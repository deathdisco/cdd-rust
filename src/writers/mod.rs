use cdd::*;

mod models;
pub use models::*;
mod requests;
pub use requests::*;

fn class_to_string(name: String, vars: Vec<Variable>) -> String {
    format!(
        "pub struct {} {{{}\n}}",
        name,
        vars.into_iter().map(var_to_string).collect::<String>()
    )
}

fn var_to_string(var: Variable) -> String {
    let var_type = if var.optional {
        format!("Option<{}>", variable_type_to_rust_type(var.variable_type))
    } else {
        format!("{}", variable_type_to_rust_type(var.variable_type))
    };

    format!("\n\tpub {}: {},", var.name, var_type)
}

fn variable_type_to_rust_type(vartype: VariableType) -> String {
    match vartype {
        VariableType::StringType => "String".to_string(),
        VariableType::BoolType => "bool".to_string(),
        VariableType::FloatType => "f64".to_string(),
        VariableType::IntType => "i32".to_string(),
        VariableType::ArrayType(t) => format!("Vec<{}>", variable_type_to_rust_type(*t)),
        VariableType::ComplexType(t) => t,
    }
}
