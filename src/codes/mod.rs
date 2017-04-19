use nickel::status::StatusCode;
use std::collections::HashMap;

pub struct Codes {
}

impl Codes {
    fn hash_map() -> HashMap<u16, StatusCode> {
        HashMap::new()
    }
}
