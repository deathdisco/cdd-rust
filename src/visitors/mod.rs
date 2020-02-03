mod requests;
pub(crate) use requests::*;
mod models;
pub(crate) use models::*;
mod struct_visitor;
// pub(crate) use struct_visitor::*;
use cdd::*;

pub(crate) fn extract_project_from_syn(ast: syn::File) -> Result<Project, failure::Error> {
    Ok(Project {
        info: None,
        models: Vec::new(),
        requests: Vec::new(),
    })
}
