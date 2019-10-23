use failure::*;
use syn::visit::Visit;
use syn::{Ident, Item, ItemStruct};

pub(crate) struct StructVisitor {
    file: syn::File,
    pub structs: Vec<String>,
}

impl StructVisitor {
    pub(crate) fn new(code: &str) -> Result<Self, failure::Error> {
        Ok(Self {
            file: syn::parse_file(&code)
                .map_err(|e| format_err!("Error parsing source code: {}", e))?,
            structs: vec![],
        })
    }

    // pub(crate) fn structs(&self) -> Vec<String> {
    //     for item in self.file.items.clone() {
    //         println!("hi");
    //     }
    //     vec![]
    // }
}

impl<'ast> Visit<'ast> for StructVisitor {
    fn visit_item(&mut self, i: &'ast Item) {
        match i {
            Item::Struct(s) => self.structs.push(format!("{}", s.ident)),
            _ => (),
        };
    }
}

#[test]
fn test_expr_parse() {
    let code = r#"
        // a comment
        struct StructOne { a: String }
        struct StructTwo;
    "#;
    let mut visitor = StructVisitor::new(&code).unwrap();
    let syntax = syn::parse_file(&code).unwrap();
    syn::visit::visit_file(&mut visitor, &syntax);
    assert_eq!(visitor.structs, vec!["StructOne", "StructTwo"]);
    // assert_eq!(visitor.structs(), vec!["StructOne", "StructTwo"]);
}
