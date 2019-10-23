use super::struct_visitor::*;
use cdd::*;

pub fn extract_models(code: &str) -> Result<Vec<Model>, failure::Error> {
    let mut visitor = StructVisitor::new(&code)?;
    let syntax = syn::parse_file(&code)?;
    syn::visit::visit_file(&mut visitor, &syntax);
    Ok(visitor
        .structs
        .into_iter()
        .map(|name| Model { name, vars: vec![] })
        .collect())
}
