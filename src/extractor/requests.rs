use cdd::*;
use std::collections::HashMap;

pub fn extract_from_ast(syntax: &syn::File) -> Result<Vec<Request>, failure::Error> {
    let mut visitor = crate::visitors::StructVisitor::new();
    syn::visit::visit_file(&mut visitor, &syntax);

    Ok(visitor
        .structs
        .into_iter()
        .map(|(name, vars)| Request {
            name,
            params: vars.into_iter().map(|v| Box::new(v)).collect(),
            method: Method::Get,
            path: "/".to_string(),
            error_type: "None".to_string(),
            response_type: "None".to_string(),
        })
        .collect())
}

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
        params: vec![],
        path: "".to_string(),
        error_type: "".to_string(),
        method: Method::Get,
        response_type: "Petty".to_string(),
    })
}

// pub fn extract_requests(code: &str) -> Vec<Request> {
//     extract_structures(code)
//         .unwrap_or(HashMap::new())
//         .into_iter()
//         .map(struct_to_request)
//         .filter_map(|r| r)
//         .collect()
// }
