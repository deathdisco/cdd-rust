use cdd::*;

pub fn print_models(models: Vec<Model>) -> String {
    let printed_models = models
        .into_iter()
        .map(model_to_string)
        .collect::<Vec<String>>()
        .join("\n\n");
    
    format!("use diesel::Queryable;\n\n{}", printed_models)
}

fn model_to_string(model: Model) -> String {
    format!(
        "#[derive(Queryable, Debug)]\n{}",
        super::class_to_string(model.name, model.vars.into_iter().map(|v| *v).collect()))
}