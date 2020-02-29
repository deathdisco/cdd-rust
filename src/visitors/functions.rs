use cdd::{Variable, VariableType};
use std::collections::HashMap;
use syn::visit::Visit;
use syn::{Fields, Item, Type};

pub(crate) struct FunctionVisitor {
    pub functions: Vec<Function>,
}

pub(crate) struct Function {
    pub name: String,
    pub params: Vec<cdd::Variable>,
    pub return_type: Option<cdd::VariableType>,
    pub variable_declarations: Vec<cdd::Variable>,
}

impl FunctionVisitor {
    pub(crate) fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for FunctionVisitor {
    // https://docs.rs/syn/1.0.16/syn/visit/trait.Visit.html
    fn visit_item(&mut self, i: &'ast Item) {
        // https://docs.rs/syn/1.0.16/syn/enum.Item.html
        match i {
            Item::Fn(f) => {
                // https://docs.rs/syn/1.0.16/syn/struct.ItemFn.html

                // let rt = f.sig.ouput;

                self.functions.push(Function {
                    name: format!("{}", f.ident),
                    params: Vec::new(),
                    return_type: None,
                    variable_declarations: Vec::new(),
                })
            }
            _ => (),
        };
    }
}