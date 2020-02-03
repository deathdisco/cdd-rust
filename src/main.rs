// use std::path::PathBuf;
// use structopt::StructOpt;

mod rustfmt;
mod util;
mod visitors;
// mod writers;
mod server;
mod models;
mod parser;
mod generator;
mod extractor;

fn main() -> Result<(), String> {
    server::start();
    Ok(())
}

// fn list_models(file: PathBuf) -> Result<(), failure::Error> {
//     let models = visitors::extract_models(&util::read_file(file)?)?;
//     let json = &serde_json::to_string(&models)?;

//     println!("{}", json);

//     Ok(())
// }

// fn list_requests(file: PathBuf) -> Result<(), failure::Error> {
//     let requests = visitors::extract_requests(&util::read_file(file)?);
//     let json = &serde_json::to_string(&requests)?;

//     println!("{}", json);

//     Ok(())
// }

// fn update_model(json: String, file: PathBuf) -> Result<(), failure::Error> {
//     let model: cdd::Model = serde_json::from_str::<cdd::Model>(&json)?;

//     let models: Vec<cdd::Model> = visitors::extract_models(&util::read_file(file.clone())?)?
//         .into_iter()
//         .map(|src_model| {
//             if model.name == src_model.name {
//                 model.clone()
//             } else {
//                 src_model
//             }
//         })
//         .collect();

//     util::write_file(file.clone(), &writers::print_models(models))?;
//     rustfmt::rustfmt(file.to_str().ok_or(failure::format_err!("bad file"))?)?;
//     Ok(())
// }
// fn update_request(json: String, file: PathBuf) -> Result<(), failure::Error> {
//     let request: cdd::Request = serde_json::from_str::<cdd::Request>(&json)?;

//     let requests: Vec<cdd::Request> = visitors::extract_requests(&util::read_file(file.clone())?)
//         .into_iter()
//         .map(|src_request| {
//             if request.name == src_request.name {
//                 request.clone()
//             } else {
//                 src_request
//             }
//         })
//         .collect();

//     util::write_file(file.clone(), &writers::print_requests(requests))?;
//     rustfmt::rustfmt(file.to_str().ok_or(failure::format_err!("bad file"))?)?;
//     Ok(())
// }

// fn insert_model(json: String, file: PathBuf) -> Result<(), failure::Error> {
//     let mut models: Vec<cdd::Model> = visitors::extract_models(&util::read_file(file.clone())?)?;
//     models.push(serde_json::from_str::<cdd::Model>(&json)?);
//     util::write_file(file.clone(), &writers::print_models(models))
// }

// fn insert_request(json: String, file: PathBuf) -> Result<(), failure::Error> {
//     let mut requests: Vec<cdd::Request> =
//         visitors::extract_requests(&util::read_file(file.clone())?);
//     requests.push(serde_json::from_str::<cdd::Request>(&json)?);
//     util::write_file(file.clone(), &writers::print_requests(requests))
// }

// fn delete_model(_name: String, _file: PathBuf) -> Result<(), failure::Error> {
//     println!("Model successfully deleted");
//     Ok(())
// }

// fn delete_request(_name: String, _file: PathBuf) -> Result<(), failure::Error> {
//     println!("Request successfully deleted");
//     Ok(())
// }

// fn generate_tests() {
//     println!("Generating tests")
// }
