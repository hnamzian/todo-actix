use serde::{Serialize};

#[derive(Serialize)]
pub struct Status {
    status: String,
}

impl Status {
    pub fn new(s: String) -> Status {
        Status {
            status: s
        }
    }
}