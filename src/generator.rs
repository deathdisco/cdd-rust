use cdd::*;

pub fn update(project: &Project, code: &str) -> Result<String, failure::Error> {
    // todo: actually update instead of generate
    Ok(crate::writers::print_models(&project.models))
}
