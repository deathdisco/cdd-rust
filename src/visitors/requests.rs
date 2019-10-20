use super::struct_visitor::*;
use cdd::*;

pub fn extract_requests(code: &str) -> Vec<Request> {
    let mut visitor = StructVisitor::new(&code).unwrap();
    let syntax = syn::parse_file(&code).unwrap();
    syn::visit::visit_file(&mut visitor, &syntax);
    visitor
        .structs()
        .into_iter()
        .map(|name| Request {
            name,
            vars: vec![],
            path: "".to_string(),
            error_type: "".to_string(),
            method: Method::Get_,
            response_type: "Petty".to_string(),
        })
        .collect()
}
