use rand::{distributions::Alphanumeric, Rng};
use serde::de::value;
use zino::Result;
use zino_core::{extension::JsonValueExt, response::Rejection, Map};

pub fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
}

pub fn str_to_usize(str: &str) -> Result<usize> {
    match str.parse::<usize>() {
        Ok(r) => Ok(r),
        Err(e) =>  return Err(Rejection::internal_server_error(e).into()),
    }
}


pub fn str_from_map(key: &str, map:&Map) -> Result<Option<String>> {
    match map.get(key) {
        Some(v) => if v.to_string_unquoted() == "null" { return Ok(None); } else { return Ok(Some(v.to_string_unquoted()));} ,
        None => return Ok(None),
    }
}