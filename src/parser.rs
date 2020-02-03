
use cdd::*;

pub(crate) fn parse_code_to_ast(code: &str) -> Result<syn::File, syn::Error> {
    syn::parse_file(&code)
}

pub(crate) fn parse_code_to_project(code: &str) -> Result<Project, failure::Error> {
    let ast: syn::File = parse_code_to_ast(code)?;
    crate::visitors::extract_project_from_syn(ast)
}
