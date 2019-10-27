use super::struct_visitor::*;
use cdd::*;

pub fn extract_models(code: &str) -> Result<Vec<Model>, failure::Error> {
    Ok(extract_structures(code)?
        .into_iter()
        .map(|(name, vars)| Model {
            name,
            vars: vars.into_iter().map(|v| Box::new(v)).collect(),
        })
        .collect())
}
