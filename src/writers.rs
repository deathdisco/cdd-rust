use crate::util;
use cdd::*;
use std::path::PathBuf;

mod models;
mod routes;

pub fn print_models(models: Vec<Model>) -> String {
    let printed_models = models
        .into_iter()
        .map(model_to_string)
        .collect::<Vec<String>>()
        .join("\n\n");
    
    format!("use diesel::Queryable;\n\n{}", printed_models)
}

pub fn print_requests(requests: Vec<Request>) -> String {
    requests
        .into_iter()
        .map(request_to_string)
        .collect::<Vec<String>>()
        .join("\n\n")
}

fn model_to_string(model: Model) -> String {
    println!("WRITING MODEL: {:#?}", model);
    format!(
        "#[derive(Queryable, Debug)]\n{}",
        class_to_string(model.name, model.vars.into_iter().map(|v| *v).collect()))
}

fn request_to_string(request: Request) -> String {
    class_to_string(request.name, request.vars.into_iter().map(|v| *v).collect())
}

fn class_to_string(name: String, vars: Vec<Variable>) -> String {
    format!(
        "struct {} {{{}}}",
        name,
        vars.into_iter().map(var_to_string).collect::<String>()
    )
}

fn var_to_string(var: Variable) -> String {
    format!(
        "{}: {},\n",
        var.name,
        variable_type_to_rust_type(var.variable_type)
    )
}

fn variable_type_to_rust_type(vartype: VariableType) -> String {
    match vartype {
        VariableType::StringType => "String".to_string(),
        VariableType::BoolType => "bool".to_string(),
        VariableType::FloatType => "f64".to_string(),
        VariableType::IntType => "i64".to_string(),
        VariableType::ArrayType(t) => format!("Vec<{}>", variable_type_to_rust_type(*t)),
        VariableType::ComplexType(t) => t,
    }
}
