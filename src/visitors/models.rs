use super::struct_visitor::*;
use cdd::*;
use std::path::PathBuf;

pub fn get_models_from_file(file: PathBuf) -> Vec<Model> {
    vec![Model {
        vars: vec![],
        name: "Guppy".to_string(),
    }]
}

pub fn extract_models(code: &str) -> Vec<Model> {
    let mut visitor = StructVisitor::new(&code).unwrap();
    let syntax = syn::parse_file(&code).unwrap();
    syn::visit::visit_file(&mut visitor, &syntax);
    visitor
        .structs()
        .into_iter()
        .map(|name| Model { name, vars: vec![] })
        .collect()
}
