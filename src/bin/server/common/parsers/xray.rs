use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

#[derive(Debug)]
enum ParseError {
    FieldMissing(String),
    UnknownFieldType { current: String, expected: String },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::FieldMissing(field) => write!(f, "Missing field: {}", field),
            ParseError::UnknownFieldType { current, expected } => write!(
                f,
                "Unknown field type: {} (expected: {})",
                current, expected
            ),
        }
    }
}

impl std::error::Error for ParseError {}

trait Parser
where
    Self: Sized,
{
    fn parse(url: &Url) -> Result<Self, ParseError>;
}

#[derive(Debug, Deserialize)]
pub enum ProxyConfig {
    Vmess(Vmess),
    Vless(Vless),
    Shadowsocks(Shadowsocks),
    Trojan(Trojan),
}

impl ProxyConfig {
    pub fn id(&self) -> Option<&str> {
        match self {
            ProxyConfig::Vless(value) => Some(&value.id),
            ProxyConfig::Vmess(value) => Some(&value.id),
            ProxyConfig::Trojan(value) => Some(&value.id),
            ProxyConfig::Shadowsocks(_) => None,
        }
    }

    pub fn password(&self) -> Option<&str> {
        match self {
            ProxyConfig::Trojan(value) => Some(&value.password),
            ProxyConfig::Shadowsocks(value) => Some(&value.password),
            _ => None,
        }
    }

    pub fn address(&self) -> &str {
        match self {
            ProxyConfig::Vless(value) => &value.address,
            ProxyConfig::Vmess(value) => &value.address,
            ProxyConfig::Trojan(value) => &value.address,
            ProxyConfig::Shadowsocks(value) => &value.address,
        }
    }

    pub fn port(&self) -> u16 {
        match self {
            ProxyConfig::Vless(value) => value.port,
            ProxyConfig::Vmess(value) => value.port,
            ProxyConfig::Trojan(value) => value.port,
            ProxyConfig::Shadowsocks(value) => value.port,
        }
    }

    pub fn protocol(&self) -> &'static str {
        match self {
            ProxyConfig::Vless(_) => "vless",
            ProxyConfig::Vmess(_) => "vmess",
            ProxyConfig::Trojan(_) => "trojan",
            ProxyConfig::Shadowsocks(_) => "ss",
        }
    }

    pub fn name(&self) -> Option<&str> {
        match self {
            ProxyConfig::Vless(value) => value.name.as_deref(),
            ProxyConfig::Vmess(value) => value.name.as_deref(),
            ProxyConfig::Trojan(value) => value.name.as_deref(),
            ProxyConfig::Shadowsocks(value) => value.name.as_deref(),
        }
    }

    pub fn extras(&self) -> &HashMap<String, String> {
        match self {
            ProxyConfig::Vless(value) => &value.extras,
            ProxyConfig::Vmess(value) => &value.extras,
            ProxyConfig::Trojan(value) => &value.extras,
            ProxyConfig::Shadowsocks(value) => &value.extras,
        }
    }

    pub fn security(&self) -> Option<&str> {
        match self {
            ProxyConfig::Vless(value) => value.security.as_deref(),
            _ => None,
        }
    }

    pub fn encryption(&self) -> Option<&str> {
        match self {
            ProxyConfig::Vless(value) => value.encryption.as_deref(),
            _ => None,
        }
    }

    pub fn network(&self) -> Option<&str> {
        match self {
            ProxyConfig::Vless(value) => Some(&value.network),
            ProxyConfig::Vmess(value) => Some(&value.network),
            _ => None,
        }
    }

    pub fn path(&self) -> Option<&str> {
        match self {
            ProxyConfig::Vless(value) => value.path.as_deref(),
            ProxyConfig::Vmess(value) => value.path.as_deref(),
            _ => None,
        }
    }

    pub fn host(&self) -> Option<&str> {
        match self {
            ProxyConfig::Vless(value) => value.host.as_deref(),
            ProxyConfig::Vmess(value) => value.host.as_deref(),
            ProxyConfig::Trojan(value) => value.host.as_deref(),
            _ => None,
        }
    }

    pub fn tls(&self) -> Option<bool> {
        match self {
            ProxyConfig::Vless(value) => Some(value.tls),
            ProxyConfig::Vmess(value) => Some(value.tls),
            _ => None,
        }
    }

    pub fn aid(&self) -> Option<u32> {
        match self {
            ProxyConfig::Vmess(value) => Some(value.aid),
            _ => None,
        }
    }

    pub fn type_field(&self) -> Option<&str> {
        match self {
            ProxyConfig::Vmess(value) => value.type_field.as_deref(),
            _ => None,
        }
    }

    pub fn method(&self) -> Option<&str> {
        match self {
            ProxyConfig::Shadowsocks(value) => Some(&value.method),
            _ => None,
        }
    }

    pub fn sni(&self) -> Option<&str> {
        match self {
            ProxyConfig::Trojan(value) => value.sni.as_deref(),
            _ => None,
        }
    }

    pub fn ws_path(&self) -> Option<&str> {
        match self {
            ProxyConfig::Trojan(value) => value.ws_path.as_deref(),
            _ => None,
        }
    }

    pub fn allow_insecure(&self) -> Option<bool> {
        match self {
            ProxyConfig::Trojan(value) => Some(value.allow_insecure),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct Vless {
    id: String,
    address: String,
    port: u16,
    security: Option<String>,
    encryption: Option<String>,
    network: String,
    path: Option<String>,
    host: Option<String>,
    tls: bool,
    name: Option<String>,
    extras: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct Vmess {
    id: String,
    address: String,
    port: u16,
    aid: u32,
    network: String,
    type_field: Option<String>,
    host: Option<String>,
    path: Option<String>,
    tls: bool,
    name: Option<String>,
    // raw parameters store
    extras: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct Shadowsocks {
    method: String,
    password: String,
    address: String,
    port: u16,
    name: Option<String>,
    extras: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct Trojan {
    id: String,
    password: String,
    address: String,
    port: u16,
    sni: Option<String>,
    ws_path: Option<String>,
    host: Option<String>,
    allow_insecure: bool,
    name: Option<String>,
    extras: HashMap<String, String>,
}

impl Parser for Vless {
    fn parse(url: &Url) -> Result<Self, ParseError> {
        let query: HashMap<_, _> = url.query_pairs().into_owned().collect();
        let mut extras = query.clone();
        extras.remove("encryption");
        extras.remove("security");

        let id = url.username().to_string();
        if id.is_empty() {
            return Err(ParseError::FieldMissing("id".to_string()));
        }

        let address = url
            .host_str()
            .ok_or(ParseError::FieldMissing("address".to_string()))?
            .to_string();

        let port = url
            .port()
            .ok_or(ParseError::FieldMissing("port".to_string()))?;

        let network = query
            .get("type")
            .ok_or(ParseError::FieldMissing("network".to_string()))?
            .to_string();

        Ok(Vless {
            id,
            address,
            port,
            network,
            security: query.get("security").cloned(),
            encryption: query.get("encryption").cloned(),
            path: query.get("path").cloned(),
            host: query.get("host").cloned(),

            name: url.fragment().map(|s| s.to_string()),
            extras,

            tls: query.get("security").map(|s| s == "tls").unwrap_or(false),
        })
    }
}

fn parse_line(url: Url) -> Result<ProxyConfig, String> {
    match url.scheme() {
        "vless" => Vless::parse(&url)
            .map(ProxyConfig::Vless)
            .map_err(|err| format!("{}", err)),
        other => Err(format!("unknown url scheme: \"{other}\"")),
    }
}

pub fn work(payload: &str) -> Result<Vec<ProxyConfig>, ()> {
    let mut configs = Vec::new();

    for line in payload.lines().take(2) {
        let Ok(url) = Url::parse(line) else {
            println!("Is not valid url {}", line);

            continue;
        };

        match parse_line(url) {
            Ok(url) => configs.push(url),
            Err(err) => println!("failed to parse line: {}", err),
        }
    }

    Ok(configs)
}
