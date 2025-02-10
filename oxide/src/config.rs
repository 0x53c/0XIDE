use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io;

#[derive(Deserialize, Serialize)]
pub struct Endpoint {
    pub method: String,
    pub path: String,
    pub handler: String,
}

pub fn load_endpoints(filepath: &str) -> Result<Vec<Endpoint>, io::Error> {
    let contents = fs::read_to_string(filepath)?;
    let endpoints: Vec<Endpoint> = serde_json::from_str(&contents)?;
    Ok(endpoints)
}
