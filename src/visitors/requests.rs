use super::struct_visitor::*;
use cdd::*;
use std::collections::HashMap;

pub fn struct_to_request(structure: (String, Vec<Variable>)) -> Option<Request> {
    let (name, _vars) = structure;

    // Some(Request {
    //     path: vars.iter().find(|&s| s.name == "path")?,
    //     error_type: vars.iter().find(|&s| s.name == "error_type")?,
    //     method: vars.iter().find(|&s| s.name == "error_type")?,
    //     response_type: vars.iter().find(|&s| s.name == "error_type")?,
    // })

    Some(Request {
        name,
        vars: vec![],
        path: "".to_string(),
        error_type: "".to_string(),
        method: Method::Get_,
        response_type: "Petty".to_string(),
    })
}

pub fn extract_requests(code: &str) -> Vec<Request> {
    extract_structures(code)
        .unwrap_or(HashMap::new())
        .into_iter()
        .map(struct_to_request)
        .filter_map(|r| r)
        .collect()
}
