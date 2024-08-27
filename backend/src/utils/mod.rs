pub mod date_utils;
pub mod oss_utils;

use rand::{distributions::Alphanumeric, Rng};
use serde::de::value;
use zino::{prelude::Validation, Result};
use zino_core::error::Error;
use zino_core::{extension::JsonValueExt, response::Rejection, warn, Map};

pub fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
}

pub fn str_to_usize(str: &str) -> Result<usize> {
    match str.parse::<usize>() {
        Ok(r) => Ok(r),
        Err(e) => return Err(Rejection::internal_server_error(e).into()),
    }
}

pub fn str_from_map(key: &str, map: &Map) -> Result<Option<String>> {
    match map.get(key) {
        Some(v) => {
            if v.to_string_unquoted() == "null" {
                return Ok(None);
            } else {
                return Ok(Some(v.to_string_unquoted()));
            }
        }
        None => return Ok(None),
    }
}

pub fn str_from_map_required(key: &str, map: &Map) -> Result<String> {
    let key_clone = key.to_string().clone();
    match map.get(key) {
        Some(v) => {
            if v.to_string_unquoted() == "null" || v.to_string_unquoted() == "" {
                let validation =
                    Validation::from_entry("err_msg", warn!("{key_clone} should provied"));
                return Err(Rejection::bad_request(validation).into());
            } else {
                return Ok(v.to_string_unquoted());
            }
        }
        None => {
            let validation = Validation::from_entry("err_msg", warn!("{key_clone} should provied"));
            return Err(Rejection::bad_request(validation).into());
        }
    }
}

pub fn usize_from_map_default(key: &str, map: &Map, default: usize) -> Result<usize> {
    let key_clone = key.to_string().clone();
    match map.get(key) {
        Some(v) => {
            if v.to_string_unquoted() == "null" {
                return Ok(1 as usize);
            } else {
                let data = v.to_string();
                match data.parse::<usize>() {
                    Ok(d) => {
                        return Ok(d);
                    }
                    Err(e) => {
                        warn!("parser {} error", key_clone);
                        return Err(Rejection::internal_server_error(e).into());
                    }
                }
            }
        }
        None => {
            return Ok(default);
        }
    }
}
