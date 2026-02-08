use std::collections::BTreeMap;

/* 
 * This file defines objects that will valid for our JSON Schemas.
 * TBD: Document the way we will diferenciate objects from types and why.
*/


#[derive(Debug, Clone)]
pub enum JSONValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Array(Vec<JSONValue>),
    Object(BTreeMap<String, JSONValue>),
}

#[derive(Debug, Clone)]
pub enum JSON {
    Object(BTreeMap<String, JSONValue>),
    Array(Vec<JSONValue>),
}

