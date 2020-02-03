use serde::{Deserialize};
use cdd::*;

#[derive(Deserialize, Debug)]
pub struct ParseRequest {
    pub code: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateRequest {
    pub code: String,
    pub project: Project,
}
