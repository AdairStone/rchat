use anyhow::anyhow;
use zino::prelude::{Query, Schema};

use crate::model::ip_detail::IpDetail;

pub struct IpService;

impl IpService {
    pub async fn ip_detail(ip: &str) -> anyhow::Result<IpDetail> {
        let query = Query::from_entry("ip", ip);
        match IpDetail::find_one::<IpDetail>(&query).await {
            Ok(resp) => {
                if resp.is_some() {
                    return Ok(resp.unwrap());
                } else {
                    let url = format!("https://ipinfo.io/{}/json", ip);
                    let response: IpDetail = reqwest::get(&url).await?.json().await?;
                    let rc = response.clone();
                    match response.insert().await {
                        Ok(_r) => {
                            return Ok(rc);
                        }
                        Err(e) => Err(anyhow!("{e:#?}")),
                    }
                }
            }
            Err(e) => Err(anyhow!("{e:#?}")),
        }
    }
}
