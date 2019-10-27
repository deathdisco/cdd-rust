use super::struct_visitor::*;
use cdd::*;

pub fn extract_models(code: &str) -> Result<Vec<Model>, failure::Error> {
    let mut visitor = StructVisitor::new();
    let syntax = syn::parse_file(&code)?;
    syn::visit::visit_file(&mut visitor, &syntax);
    Ok(visitor
        .structs
        .into_iter()
        .map(|(name, vars)| Model { name, vars: vec![] })
        .collect())
}
