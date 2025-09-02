use luxnulla::CONFIG_DIR;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    pub config: JsonValue,
}

impl Group {
    pub fn new(name: String, config: JsonValue) -> Self {
        Self { name, config }
    }
}

#[derive(Debug, Clone)]
pub struct StorageService {
    groups: Arc<RwLock<HashMap<String, Group>>>,
    groups_dir: Option<PathBuf>,
}

impl StorageService {
    pub fn new() -> Self {
        let config_dir_path = eyre::OptionExt::ok_or_eyre(dirs::config_dir(), "cannot get a dir")
            .unwrap()
            .join(CONFIG_DIR);

        let groups_dir = config_dir_path.join("groups");

        // Создаем директорию, если она не существует
        if !groups_dir.exists() {
            fs::create_dir_all(&groups_dir)
                .unwrap_or_else(|e| panic!("Failed to create groups directory: {}", e));
        }

        let instance = Self {
            groups: Arc::new(RwLock::new(HashMap::new())),
            groups_dir: Some(groups_dir),
        };

        if let Err(e) = instance.load_groups_from_disk() {
            eprintln!("Warning: Could not load groups from disk: {}", e);
        }

        instance
    }

    fn load_groups_from_disk(&self) -> Result<(), StorageError> {
         if let Some(groups_dir) = &self.groups_dir {
             let mut groups = self.groups.write().map_err(|_| StorageError::LockError)?;

             for entry in fs::read_dir(groups_dir).map_err(|e| StorageError::FileError(e.to_string()))? {
                 let entry = match entry {
                     Ok(e) => e,
                     Err(e) => {
                         eprintln!("Warning: Failed to read directory entry: {}", e);
                         continue; // Пропускаем "сломанную" запись и продолжаем
                     }
                 };

                 let path = entry.path();
                 if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                     match fs::read_to_string(&path) {
                         Ok(content) => match serde_json::from_str::<Group>(&content) {
                             Ok(group) => {
                                 groups.insert(group.name.clone(), group);
                             }
                             Err(e) => {
                                 eprintln!("Warning: Failed to parse group file at {:?}: {}", path, e);
                             }
                         },
                         Err(e) => {
                             eprintln!("Warning: Failed to read group file at {:?}: {}", path, e);
                         }
                     }
                 }
             }
         }
         Ok(())
     }

    pub fn store_group(&self, group: Group) -> Result<(), StorageError> {
        let group_name = group.name.clone();
        {
            let mut groups = self.groups.write().map_err(|_| StorageError::LockError)?;
            groups.insert(group.name.clone(), group);
        }
        self.save_group_to_file(&group_name)?;
        Ok(())
    }

    pub fn get_group(&self, name: &str) -> Result<Option<Group>, StorageError> {
        let groups = self.groups.read().map_err(|_| StorageError::LockError)?;
        Ok(groups.get(name).cloned())
    }

    pub fn get_all_groups(&self) -> Result<Vec<Group>, StorageError> {
        let groups = self.groups.read().map_err(|_| StorageError::LockError)?;

        Ok(groups.values().cloned().collect())
    }

    pub fn update_group_config(
        &self,
        name: &str,
        new_config: JsonValue,
    ) -> Result<bool, StorageError> {
        let updated = {
            let mut groups = self.groups.write().map_err(|_| StorageError::LockError)?;
            if let Some(group) = groups.get_mut(name) {
                group.config = new_config;
                true
            } else {
                false
            }
        };
        if updated {
            self.save_group_to_file(name)?;
        }
        Ok(updated)
    }

    pub fn delete_group(&self, name: &str) -> Result<bool, StorageError> {
        let result = {
            let mut groups = self.groups.write().map_err(|_| StorageError::LockError)?;
            groups.remove(name).is_some()
        };
        if result {
            self.delete_group_file(name)?;
        }
        Ok(result)
    }

    pub fn group_exists(&self, name: &str) -> Result<bool, StorageError> {
        let groups = self.groups.read().map_err(|_| StorageError::LockError)?;
        Ok(groups.contains_key(name))
    }

    pub fn count_groups(&self) -> Result<usize, StorageError> {
        let groups = self.groups.read().map_err(|_| StorageError::LockError)?;
        Ok(groups.len())
    }

    pub fn list_group_names(&self) -> Result<Vec<String>, StorageError> {
        let groups = self.groups.read().map_err(|_| StorageError::LockError)?;
        Ok(groups.keys().cloned().collect())
    }

    pub fn upsert_group(&self, group: Group) -> Result<bool, StorageError> {
        let group_name = group.name.clone();
        let existed = {
            let mut groups = self.groups.write().map_err(|_| StorageError::LockError)?;
            let existed = groups.contains_key(&group.name);
            groups.insert(group.name.clone(), group);
            existed
        };
        self.save_group_to_file(&group_name)?;
        Ok(existed)
    }

    pub fn clear_all_groups(&self) -> Result<(), StorageError> {
        {
            let mut groups = self.groups.write().map_err(|_| StorageError::LockError)?;
            groups.clear();
        }
        self.clear_all_group_files()?;
        Ok(())
    }

    pub fn save_group_to_file(&self, group_name: &str) -> Result<(), StorageError> {
        let groups = self.groups.read().map_err(|_| StorageError::LockError)?;

        match groups.get(group_name) {
            Some(group) => {
                let groups_dir = match &self.groups_dir {
                    Some(val) => val.clone(),
                    None => PathBuf::new(),
                };

                let file_path = groups_dir.join(format!("{}.json", group_name));

                let json_data = serde_json::to_string_pretty(group)
                    .map_err(|e| StorageError::SerializationError(e.to_string()))?;
                fs::write(file_path, json_data)
                    .map_err(|e| StorageError::FileError(e.to_string()))?;

                Ok(())
            }
            None => Err(StorageError::GroupNotFound(group_name.to_string())),
        }
    }

    pub fn delete_group_file(&self, group_name: &str) -> Result<(), StorageError> {
        if let Some(ref groups_dir) = self.groups_dir {
            let file_path = Path::new(groups_dir).join(format!("{}.json", group_name));
            if file_path.exists() {
                fs::remove_file(file_path).map_err(|e| StorageError::FileError(e.to_string()))?;
            }
        }
        Ok(())
    }

    pub fn clear_all_group_files(&self) -> Result<(), StorageError> {
        if let Some(ref groups_dir) = self.groups_dir {
            let dir_path = Path::new(groups_dir);
            if dir_path.exists() {
                for entry in
                    fs::read_dir(dir_path).map_err(|e| StorageError::FileError(e.to_string()))?
                {
                    let entry = entry.map_err(|e| StorageError::FileError(e.to_string()))?;
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                        fs::remove_file(path)
                            .map_err(|e| StorageError::FileError(e.to_string()))?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl Default for StorageService {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Failed to acquire lock on storage")]
    LockError,

    #[error("Group '{0}' not found")]
    GroupNotFound(String),

    #[error("Invalid JSON configuration: {0}")]
    InvalidJson(String),

    #[error("Storage operation failed: {0}")]
    OperationFailed(String),

    #[error("File operation failed: {0}")]
    FileError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}
