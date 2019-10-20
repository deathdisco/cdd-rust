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
    #[structopt(name = "add-model", about = "Add a model in a source file")]
    AddModel {
        #[structopt(help = "", parse(from_os_str), name = "file")]
        file: PathBuf,
        #[structopt(help = "", name = "json")]
        json: String,
    },
    #[structopt(name = "add-request", about = "Add a Request in a source file")]
    AddRequest {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let code = r#"struct Test;"#;
    // let visitor = visitors::StructVisitor::new(code);

    match Command::from_args() {
        Command::Error => {
            eprintln!("An example error to test things with");
            eprintln!("A second line");
            std::process::exit(1);
        }
        Command::ListModels { file } => list_models(file)?,
        Command::ListRequests { file } => list_requests(file),
        Command::UpdateModel { file, json } => update_model(json),
        Command::UpdateRequest { file, json } => update_request(json),
        Command::AddModel { file, json } => add_model(json),
        Command::AddRequest { file, json } => add_request(json),
        Command::DeleteModel { file, name } => delete_model(name),
        Command::DeleteRequest { file, name } => delete_request(name),
        _ => {
            eprintln!("Unsupported or unknown operation");
            std::process::exit(1);
        }
    }

    Ok(())
}

fn list_models(file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let models = visitors::extract_models(&util::read_file(file)?);
    let json = &serde_json::to_string(&models)?;

    println!("{}", json);

    Ok(())
}

fn list_requests(file: PathBuf) {
    let reply = r#"
        [{
            "method": "GET",
            "name": "/v1/pets",
            "response_type": "Pet",
            "error_type": "Error",
            "vars": []
        }]
    "#;
    println!("{}", reply);
}

fn update_model(json: String) {
    println!("json file")
}
fn update_request(json: String) {
    println!("json file")
}
fn add_model(json: String) {
    println!("json file")
}
fn add_request(json: String) {
    println!("json file")
}
fn delete_model(name: String) {
    println!("Model successfully deleted")
}
fn delete_request(name: String) {
    println!("Request successfully deleted")
}
