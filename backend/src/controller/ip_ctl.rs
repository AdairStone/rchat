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
    let ip: String = str_from_map_required("ip", &body)?;
    match IpService::ip_detail(&ip).await {
        Ok(detail) => {
            res.set_json_data(json!(detail));
            Ok(res.into())
        }
        Err(e) => {
            return Err(Rejection::from_error(warn!("error for ip detail: {e:#?} ")).into());
        }
    }
}
