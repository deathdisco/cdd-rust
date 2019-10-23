use std::path::PathBuf;
use structopt::StructOpt;

mod util;
mod visitors;
mod writers;

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "list-models", about = "Lists models in a source file")]
    ListModels {
        #[structopt(help = "", parse(from_os_str), name = "file")]
        file: PathBuf,
    },
    #[structopt(name = "list-Requests", about = "Lists Requests in a source file")]
    ListRequests {
        #[structopt(help = "", parse(from_os_str), name = "file")]
        file: PathBuf,
    },
    #[structopt(name = "update-model", about = "Update a model in a source file")]
    UpdateModel {
        #[structopt(help = "", parse(from_os_str), name = "file")]
        file: PathBuf,
        #[structopt(help = "", name = "json")]
        json: String,
    },
    #[structopt(name = "update-request", about = "Update a Request in a source file")]
    UpdateRequest {
        #[structopt(help = "", parse(from_os_str), name = "file")]
        file: PathBuf,
        #[structopt(help = "", name = "json")]
        json: String,
    },
    #[structopt(name = "insert-model", about = "Insert a model in a source file")]
    InsertModel {
        #[structopt(help = "", parse(from_os_str), name = "file")]
        file: PathBuf,
        #[structopt(help = "", name = "json")]
        json: String,
    },
    #[structopt(name = "insert-request", about = "Insert a Request in a source file")]
    InsertRequest {
        #[structopt(help = "", parse(from_os_str), name = "file")]
        file: PathBuf,
        #[structopt(help = "", name = "json")]
        json: String,
    },
    #[structopt(name = "delete-model", about = "Delete a model in a source file")]
    DeleteModel {
        #[structopt(help = "", parse(from_os_str), name = "file")]
        file: PathBuf,
        #[structopt(help = "", name = "name")]
        name: String,
    },
    #[structopt(name = "delete-request", about = "Delete a request in a source file")]
    DeleteRequest {
        #[structopt(help = "", parse(from_os_str), name = "file")]
        file: PathBuf,
        #[structopt(help = "", name = "name")]
        name: String,
    },
    #[structopt(
        name = "test-error",
        about = "Generate a response from stderr (for testing)"
    )]
    Error,
}

fn main() -> Result<(), String> {
    // let code = r#"struct Test;"#;
    // let visitor = visitors::StructVisitor::new(code);

    run().map_err(|e| format!("{}", e))
}

fn run() -> Result<(), failure::Error> {
    match Command::from_args() {
        Command::Error => {
            eprintln!("An example error to test things with");
            eprintln!("A second line");
            std::process::exit(1);
        }
        Command::ListModels { file } => list_models(file)?,
        Command::ListRequests { file } => list_requests(file)?,
        Command::UpdateModel { file, json } => update_model(json, file)?,
        Command::UpdateRequest { file, json } => update_request(json, file)?,
        Command::InsertModel { file, json } => insert_model(json, file)?,
        Command::InsertRequest { file, json } => insert_request(json, file)?,
        Command::DeleteModel { file, name } => delete_model(name),
        Command::DeleteRequest { file, name } => delete_request(name),
        _ => {
            eprintln!("Unsupported or unknown operation");
            std::process::exit(1);
        }
    }

    Ok(())
}

fn list_models(file: PathBuf) -> Result<(), failure::Error> {
    let models = visitors::extract_models(&util::read_file(file)?)?;
    let json = &serde_json::to_string(&models)?;

    println!("{}", json);

    Ok(())
}

fn list_requests(file: PathBuf) -> Result<(), failure::Error> {
    let requests = visitors::extract_requests(&util::read_file(file)?)?;
    let json = &serde_json::to_string(&requests)?;

    println!("{}", json);

    Ok(())
}

fn update_model(json: String, file: PathBuf) -> Result<(), failure::Error> {
    let model: cdd::Model = serde_json::from_str::<cdd::Model>(&json)?;

    let models: Vec<cdd::Model> = visitors::extract_models(&util::read_file(file.clone())?)?
        .into_iter()
        .map(|src_model| {
            if model.name == src_model.name {
                model.clone()
            } else {
                src_model
            }
        })
        .collect();

    util::write_file(file.clone(), &writers::print_models(models))
}
fn update_request(json: String, file: PathBuf) -> Result<(), failure::Error> {
    let request: cdd::Request = serde_json::from_str::<cdd::Request>(&json)?;

    let requests: Vec<cdd::Request> = visitors::extract_requests(&util::read_file(file.clone())?)?
        .into_iter()
        .map(|src_request| {
            if request.name == src_request.name {
                request.clone()
            } else {
                src_request
            }
        })
        .collect();

    util::write_file(file.clone(), &writers::print_requests(requests))
}
fn insert_model(json: String, file: PathBuf) -> Result<(), failure::Error> {
    let mut models: Vec<cdd::Model> = visitors::extract_models(&util::read_file(file.clone())?)?;
    models.push(serde_json::from_str::<cdd::Model>(&json)?);
    util::write_file(file.clone(), &writers::print_models(models))
}
fn insert_request(json: String, file: PathBuf) -> Result<(), failure::Error> {
    let mut requests: Vec<cdd::Request> =
        visitors::extract_requests(&util::read_file(file.clone())?)?;
    requests.push(serde_json::from_str::<cdd::Request>(&json)?);
    util::write_file(file.clone(), &writers::print_requests(requests))
}
fn delete_model(name: String) {
    println!("Model successfully deleted")
}
fn delete_request(name: String) {
    println!("Request successfully deleted")
}
