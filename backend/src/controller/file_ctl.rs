use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use rand::Rng;
use zino::{prelude::*, Cluster, Request, Response, Result};

use crate::utils::{date_utils::{current_date_ymd, current_s}, oss_utils::OSS_CLIENT, str_from_map_required};

pub async fn upload(mut req: Request) -> Result {
    let (mut body, files) = req.parse_form_data::<Map>().await?;
    let mut uploads = Vec::new();
    for mut file in files {
        if let Some(file_name) = file.file_name() {
            let current_date = current_date_ymd();
            let current_s = current_s();
            let random = rand::thread_rng().gen_range(1..101);
            let new_file_name = format!("{}{}_{}",current_s,random, file_name);
            let key = format!("/ada_chat/files/{}/{}", current_date, new_file_name);
            match OSS_CLIENT
                .upload_file_bytes(&key, &file.bytes().to_vec())
                .await
            {
                Ok(file_url) => {
                    let mut map = Map::new();
                    tracing::info!("upload file: {:?}", file_url);
                    map.upsert("url", file_url);
                    map.upsert("field_name", file.field_name());
                    map.upsert("file_name", file_name);
                    map.upsert("file_key", key);
                    uploads.push(map);
                }
                Err(e) => {
                    tracing::error!("{:#?}", e);
                    return Err(
                        Rejection::with_message(format!("upload file error: {:?}", e)).into(),
                    );
                }
            }
        }
    }
    body.upsert("files", uploads);
    let mut res = Response::default().context(&req);
    res.set_json_data(Map::data_entry(body));
    Ok(res.into())
}


pub async fn delete_file(mut req: Request) -> Result{
    let mut res = Response::default().context(&req);
    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("message".to_owned(), "删除成功".to_owned());
    data.insert("success".to_owned(), "true".to_owned());
    res.set_json_data(json!(data));
    Ok(res.into())
}

pub async fn get_file_url(mut req: Request) -> Result {
    let body = req.parse_body::<Map>().await?;
    let mut res = Response::default().context(&req);
    let file_key = str_from_map_required("file_key", &body)?;
    // let _shop_id_uid = Uuid::from_str(&shop_id_str).extract(&req)?;
    let mut data: HashMap<String, String> = HashMap::new();
    match OSS_CLIENT.generate_signed_url(&file_key, 60 * 30).await {
        Ok(url) => {
            data.insert("url".to_string(), url);
        }
        Err(e) => {
            return Err(Rejection::with_message(format!("get file url error: {:?}", e)).into());
        }
    }
    data.insert("file_key".to_owned(), file_key);
    res.set_json_data(json!(data));
    Ok(res.into())
}

