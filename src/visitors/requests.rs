use syn::visit::Visit;
use syn::{Ident, Item, ItemStruct};

pub(crate) struct StructVisitor {
    file: syn::File,
    structs: Vec<String>,
}

impl StructVisitor {
    pub(crate) fn new(code: &str) -> Result<Self, String> {
        Ok(Self {
            file: syn::parse_file(&code).map_err(|_| "bad file".to_string())?,
            structs: vec![],
        })
    }

    pub(crate) fn structs(&self) -> Vec<String> {
        for item in self.file.items.clone() {
            println!("hi");
        }
        vec![]
    }
}

impl<'ast> Visit<'ast> for StructVisitor {
    fn visit_item(&mut self, i: &'ast Item) {
        println!("===> {:?}", i.clone());
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
    assert_eq!(visitor.structs(), vec!["StructOne", "StructTwo"]);
}
