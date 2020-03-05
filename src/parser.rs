use cdd::*;

pub(crate) fn parse_code_to_ast(code: &str) -> Result<syn::File, syn::Error> {
    syn::parse_file(&code)
}

pub(crate) fn parse_code_to_project(code: &str) -> Result<Project, failure::Error> {
    let ast: syn::File = parse_code_to_ast(code)?;
    crate::extractor::extract_project_from_syn(&ast)
}

pub(crate) fn parse_code_to_json(code: &str) -> Result<String, failure::Error> {
    let syn_file: syn::File = syn::parse_file(&code)?;
    Ok(syn_serde::json::to_string_pretty(&syn_file))
}

pub(crate) fn parse_ast_to_code(ast: &serde_json::Value) -> Result<String, failure::Error> {
    let stringified_json = ast.to_string();
    let syntax: syn::File = syn_serde::json::from_str(&stringified_json)?;

    use quote::ToTokens;
    
    let code = syntax.into_token_stream().to_string();
    let formatted_code = crate::rustfmt::rustfmt(&code)?;

    Ok(formatted_code)
}