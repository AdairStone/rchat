use std::{fs::File, time::UNIX_EPOCH};
use std::io::Read;
use std::time::{Duration, SystemTime};
use std::error::Error;
use opendal::{Builder, Configurator, Operator};
use opendal::services::Oss;
use reqwest::Client;
use serde::Deserialize;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use urlencoding::encode as url_encode;
use base64::{engine::general_purpose, Engine};
use anyhow::anyhow;
use anyhow::Result;

use crate::app_config::SETTINGS;

lazy_static! {
    pub static ref OSS_CLIENT: OssClient = {
        let config = OssConfig {
            access_key_id: SETTINGS.oss_access_key_id.clone(),
            access_key_secret: SETTINGS.oss_access_key_secret.clone(),
            endpoint: SETTINGS.oss_endpoint.clone(),
            bucket_name: SETTINGS.oss_bucket_name.clone(),
        };
        OssClient::new(config)
    };
}

#[derive(Deserialize)]
pub struct OssConfig {
    access_key_id: String,
    access_key_secret: String,
    endpoint: String,
    bucket_name: String,
}

pub struct OssClient {
    client: Operator,
    config: OssConfig,
}


impl OssClient {
    pub fn new(config: OssConfig) -> Self {
        let oss_backend = Oss::default()
        .bucket(&config.bucket_name)
        .endpoint(&config.endpoint)
        .access_key_id(&config.access_key_id)
        .access_key_secret(&config.access_key_secret);
        let op = Operator::new(oss_backend).unwrap().finish();
        Self {
            client: op,
            config,
        }
    }

    fn generate_signature(&self, string_to_sign: &str) -> String {
        let mut mac = Hmac::<Sha1>::new_from_slice(self.config.access_key_secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(string_to_sign.as_bytes());
        let result = mac.finalize();
        general_purpose::STANDARD.encode(result.into_bytes())
    }

    pub async fn upload_file(&self, key: &str, file_path: &str) -> Result<String> {
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        OSS_CLIENT.client.write(key, buffer).await?;
        let presign = OSS_CLIENT.client.presign_read(key, Duration::from_secs(60*30)).await?;
        Ok(presign.uri().to_string())
    }

    pub async fn upload_file_bytes(&self, key: &str, file_bytes: &[u8]) -> Result<String> {
        let bytes = file_bytes.to_owned();
        OSS_CLIENT.client.write(key, bytes).await?;
        let url = format!("https://{}.{}{}", &self.config.bucket_name, &self.config.endpoint, &key);
        Ok(url)
    }

    pub async fn generate_signed_url(&self, key: &str, expires_in: u64) -> Result<String> {
        let presign = OSS_CLIENT.client.presign_read(key, Duration::from_secs(expires_in)).await?;
        Ok(presign.uri().to_string())
    }
}
