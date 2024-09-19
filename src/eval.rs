use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum LispValue {
    LString(String),
    LInteger(i64),
    LFloat(f64),
    LBool(bool),
    List(Vec<LispValue>),
    LMap(HashMap<String, LispValue>),
    LFn(String),
}
