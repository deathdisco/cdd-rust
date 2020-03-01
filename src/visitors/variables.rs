use cdd::{Variable, VariableType};
use std::collections::HashMap;
use syn::visit::Visit;
use syn::{Fields, Item, Type};

pub(crate) fn find_typepath_in_type(ty: &syn::Type) -> Option<&Path> {
    match *ty {
        syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
        _ => None,
    }
}

//     fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
//         let idents_of_path = path
//             .segments
//             .iter()
//             .into_iter()
//             .fold(String::new(), |mut acc, v| {
//                 acc.push_str(&v.ident.to_string());
//                 acc.push('|');
//                 acc
//             });
//         vec!["Option|", "std|option|Option|", "core|option|Option|"]
//             .into_iter()
//             .find(|s| &idents_of_path == *s)
//             .and_then(|_| path.segments.last())
//     }

use syn::{GenericArgument, Path, PathArguments, PathSegment};

// finds a path segment eg vec!["ApiResult|", "cdd|ApiResult|"])
pub(crate) fn find_path_segment(path: &Path, matches: Vec<&str>) -> Option<PathSegment> {
    let idents_of_path = path
        .segments
        .iter()
        .into_iter()
        .fold(String::new(), |mut acc, v| {
            acc.push_str(&v.ident.to_string());
            acc.push('|');
            acc
        });
    matches
        .into_iter()
        .find(|s| &idents_of_path == *s)
        .and_then(|_| path.segments.last())
        .map(|ps| ps.clone())
}

// // Extracts T from ApiResult<T>
// pub(crate) fn unwrap_functor(path_segment: Option<PathSegment>) -> Option<Type> {
//     path_segment
//         // .and_then(|pair_path_segment| {
//         //     let type_params = pair_path_segment.arguments.clone();
//         //     // It should have only on angle-bracketed param ("<String>"):
//         //     match type_params {
//         //         PathArguments::AngleBracketed(params) => params.args.clone().first().clone(),
//         //         _ => None,
//         //     }
//         // })
//         .and_then(|generic_arg| match generic_arg {
//             GenericArgument::Type(ref ty) => Some(ty),
//             _ => None,
//         })
//         .map(|ty| ty.clone())
// }