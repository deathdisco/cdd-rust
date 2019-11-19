use cdd::{Variable, VariableType};
use failure::*;
use std::collections::HashMap;
use syn::visit::Visit;
use syn::{Fields, FieldsNamed, Ident, Item, ItemStruct, Type};

pub fn extract_structures(code: &str) -> Result<HashMap<String, Vec<Variable>>, failure::Error> {
    let mut visitor = StructVisitor::new();
    let syntax = syn::parse_file(&code)?;
    syn::visit::visit_file(&mut visitor, &syntax);

    Ok(visitor.structs)
}

fn syn_type_to_cdd_type(ty: &str) -> VariableType {
    match ty {
        "String" => VariableType::StringType,
        "f64" => VariableType::FloatType,
        _ => VariableType::BoolType,
    }
}

fn extract_variables(fields: &Fields) -> Vec<cdd::Variable> {
    let mut vars = Vec::new();

    for field in fields.iter() {
        if let Some(ident) = &field.ident {
            if let Type::Path(ty) = &field.ty {
                for segment in &ty.path.segments {
                    // println!("var: {}: {}", ident, segment.ident);
                    let name = format!("{}", ident);
                    let ty = format!("{}", segment.ident);

                    vars.push(cdd::Variable {
                        name,
                        optional: false,
                        value: None,
                        variable_type: syn_type_to_cdd_type(&ty),
                    });
                }
            }
        }
    }
    vars
}

pub(crate) struct StructVisitor {
    pub structs: HashMap<String, Vec<cdd::Variable>>,
}

impl StructVisitor {
    pub(crate) fn new() -> Self {
        Self {
            structs: HashMap::new(),
        }
    }
}

impl<'ast> Visit<'ast> for StructVisitor {
    fn visit_item(&mut self, i: &'ast Item) {
        match i {
            Item::Struct(s) => {
                let struct_name = format!("{}", s.ident);
                let variables = extract_variables(&s.fields);
                self.structs.insert(struct_name, variables);
            }
            _ => (),
        };
    }
}

#[test]
fn test_expr_parse() {
    let code = r#"
        // a comment
        struct StructOne { example: String }
        struct StructTwo;
    "#;
    let mut visitor = StructVisitor::new();
    let syntax = syn::parse_file(&code).unwrap();
    syn::visit::visit_file(&mut visitor, &syntax);

    assert!(visitor.structs.contains_key("StructOne"));
    assert!(visitor.structs.contains_key("StructTwo"));
    assert_eq!(visitor.structs.len(), 2);
    assert_eq!(visitor.structs["StructOne"].len(), 1);
    assert_eq!(visitor.structs["StructTwo"].len(), 0);
}

#[test]
fn test_var_parse() {
    let code = r#"
        struct Dog {
            name: String,
            age: f64,
        }
    "#;
    let mut visitor = StructVisitor::new();
    let syntax = syn::parse_file(&code).unwrap();
    syn::visit::visit_file(&mut visitor, &syntax);

    assert!(visitor.structs.contains_key("Dog"));
    assert_eq!(visitor.structs.len(), 1);
    assert_eq!(visitor.structs["Dog"].len(), 2);

    let dog_name: cdd::Variable = visitor.structs["Dog"]
        .iter()
        .find(|v| v.name == "name")
        .unwrap()
        .clone();

    assert_eq!(dog_name.variable_type, cdd::VariableType::StringType);

    let dog_age: cdd::Variable = visitor.structs["Dog"]
        .iter()
        .find(|&v| v.name == "age")
        .unwrap()
        .clone();

    assert_eq!(dog_age.variable_type, cdd::VariableType::FloatType);
}
