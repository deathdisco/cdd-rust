use cdd::*;

pub(crate) fn parse_code_to_ast(code: &str) -> Result<syn::File, syn::Error> {
    syn::parse_file(&code)
}

pub(crate) fn parse_code_to_project(code: &str) -> Result<Project, failure::Error> {
    let ast: syn::File = parse_code_to_ast(code)?;
    crate::extractor::extract_project_from_syn(&ast)
}

use syn_serde::json;
pub(crate) fn parse_code_to_json(code: &str) -> Result<String, failure::Error> {
    let syn_file: syn::File = syn::parse_file(&code)?;
    Ok(json::to_string_pretty(&syn_file))
}

pub(crate) fn parse_json_to_code(code: &str) -> Result<String, failure::Error> {
    let syn_file: syn::File = json::from_str(&code)?;
    Ok(format!("{:?}", syn_file))
}