use std::collections::HashMap;

use crate::eval::LispValue;

pub struct Env {
    pub env: HashMap<String, LispValue>,
}

impl Env {
    pub fn new(built_in: HashMap<String, LispValue>) -> Self {
        Self {
            env: built_in.clone(),
        }
    }
}
