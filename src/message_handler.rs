extern crate curl;
extern crate json;

use curl::easy::{Easy2, Handler, WriteError};
use std::collections::hash_map::HashMap;
use json::Value;

pub struct MessageHandler {
    matches: HashMap<String, String>,
    message: Option<Value>,
}

impl MessageHandler {
    pub fn new() -> MessageHandler {
        
    }
}

impl Handler for MessageHandler {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {

    }
}
