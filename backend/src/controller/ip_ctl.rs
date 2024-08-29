use crate::service::ip_service::IpService;
use crate::utils::str_from_map_required;
use std::collections::HashMap;
use zino::prelude::{Rejection, RequestContext};
use zino::{Request, Response, Result};
use zino_core::error::Error;
use zino_core::{json, warn, Map};

pub async fn ip_detail(mut req: Request) -> Result {
    let body = req.parse_body::<Map>().await?;

    let mut res = Response::default().context(&req);
    let mut data: HashMap<String, String> = HashMap::new();

    let ip: String = str_from_map_required("ip", &body)?;
    match IpService::ip_detail(&ip).await {
        Ok(detail) => {
            data.insert("message".to_owned(), "删除成功".to_owned());
            data.insert("success".to_owned(), "true".to_owned());
            res.set_json_data(json!(data));
            Ok(res.into())
        }
        Err(e) => {
            return Err(Rejection::from_error(warn!("error for ip detail: {e:#?} ")).into());
        }
    }
}
