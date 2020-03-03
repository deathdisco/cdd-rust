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

                

                // let return_type = match f.sig.output {
                //     Type(_, ty) => {
                //         crate::visitors::syn_type_to_cdd_type(ty)
                //     },
                //     Default => None,
                // };

                self.functions.push(Function {
                    name: format!("{}", f.sig.ident),
                    params: Vec::new(),
                    return_type: extract_api_result_type(f.sig.output.clone()),
                    variable_declarations: Vec::new(),
                })
            }
            _ => (),
        };
    }
}

// extract (U, T) from U<T>
fn unwrap_type_from_functor(ty: syn::Type) -> Option<(syn::Ident, Option<syn::Type>)> {
    // https://docs.rs/syn/1.0.16/syn/enum.Type.html

    // refactor
    match ty {
        syn::Type::Path(typepath) => {
            // println!("typepath:{:?}", typepath.clone().path.segments);
            for path_segment in typepath.path.segments {
                // PathSegment
                // println!("ARGS:{:?}", path_segment.arguments);
                // println!("PATH FOUND {:?}", path_segment.ident);

                match path_segment.arguments {
                    // https://docs.rs/syn/1.0.16/syn/enum.PathArguments.html
                    syn::PathArguments::AngleBracketed(abga) => {
                        // AngleBracketedGenericArguments
                        for ga in abga.args {
                            if let syn::GenericArgument::Type(t) = ga {
                                // println!("--> {:?}", t);
                                return Some((path_segment.ident, Some(t)));
                            }
                        }
                    },
                    syn::PathArguments::None => {
                        return Some((path_segment.ident, None));
                    }
                    _ => ()
                }
            }
        },
        _ => ()
    }
    None
}

use super::*;
// extract T from ApiResult<T>
fn extract_api_result_type(path: syn::ReturnType) -> Option<VariableType> {
    // https://docs.rs/syn/1.0.16/syn/enum.Type.html
    if let syn::ReturnType::Type(_r_arrow, ty) = path {
        if let Some((ident, inner_type)) = unwrap_type_from_functor(*ty) {
            // found a functor, check its ident is ApiResult
            if format!("{}", ident) == "ApiResult".to_string() {
                // check if it had content
                if let Some(inner_type) = inner_type {
                    if let Some((ident, _)) = unwrap_type_from_functor(inner_type) {
                        return Some(syn_type_to_cdd_type(&format!("{}", ident)));
                    }
                }
            }
        }
    }
    None
}
