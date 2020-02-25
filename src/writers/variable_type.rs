use cdd::*;

pub(crate) fn variable_type_to_rust_type(vt: &VariableType) -> String {
    match(vt) {
        VariableType::StringType => "String".to_string(),
        VariableType::IntType => "i64".to_string(),
        VariableType::BoolType => "bool".to_string(),
        VariableType::FloatType => "f64".to_string(),
        VariableType::ArrayType(vt) => "Vec<_>".to_string(),
        VariableType::ComplexType(ty) => ty.clone(),
    }
}
