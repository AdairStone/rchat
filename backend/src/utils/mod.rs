use rand::{distributions::Alphanumeric, Rng};
use zino::Result;
use zino_core::response::Rejection;

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