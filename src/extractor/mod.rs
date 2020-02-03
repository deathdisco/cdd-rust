use cdd::*;

mod models;

pub(crate) fn extract_project_from_syn(ast: syn::File) -> Result<Project, failure::Error> {
    Ok(Project {
        info: None,
        models: models::extract_from_ast(ast)?,
        requests: Vec::new(),
    })
}
