use once_cell::sync::Lazy;

use std::sync::Mutex;
use std::collections::HashMap;


//use serde_json::json;

#[derive(Debug,Clone, Eq, Hash, PartialEq)]
pub struct StoredURL {
    pub id: String,
    pub https_address: String,
}

impl std::fmt::Display for StoredURL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.https_address)
    }
}

pub static DATA: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(
    HashMap::from([
        
    ])
));