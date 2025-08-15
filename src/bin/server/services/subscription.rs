use crate::common::parsers::{self, xray::ProxyConfig};

pub async fn get_configs(url: &str) -> Result<Vec<ProxyConfig>, ()> {
    println!("--- Fetching from plain text URL: {} ---", url);

    match parsers::subscribe::fetch(url).await {
        Ok(body) => match parsers::subscribe::parse(body.as_str()).await {
            Ok(raw_subs) => {
                let subs = parsers::xray::work(raw_subs.as_str());

                Ok(subs.unwrap())
            }
            Err(_) => Err(()),
        },
        Err(_) => Err(()),
    }
}
