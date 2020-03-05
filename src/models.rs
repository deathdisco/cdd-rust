use cdd::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ParseRequest {
    pub code: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateRequest {
    pub code: String,
    pub project: Project,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SerialiseRequest {
    pub code: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DeserialiseRequest {
    pub ast: serde_json::Value,
}