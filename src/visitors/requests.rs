use super::struct_visitor::*;
use cdd::*;

pub fn extract_requests(code: &str) -> Result<Vec<Request>, failure::Error> {
    let mut visitor = StructVisitor::new();
    let syntax = syn::parse_file(&code)?;
    syn::visit::visit_file(&mut visitor, &syntax);
    Ok(visitor
        .structs
        .into_iter()
        .map(|(name, vars)| Request {
            name,
            vars: vec![],
            path: "".to_string(),
            error_type: "".to_string(),
            method: Method::Get_,
            response_type: "Petty".to_string(),
        })
        .collect())
}
