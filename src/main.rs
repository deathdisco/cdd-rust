mod visitors;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "list-models", about = "Lists models in a source file")]
    ListModels {
        #[structopt(help = "", parse(from_os_str), name = "file")]
        file: PathBuf,
    },
    #[structopt(name = "list-routes", about = "Lists routes in a source file")]
    ListRoutes {
        #[structopt(help = "", parse(from_os_str), name = "file")]
        file: PathBuf,
    },
    #[structopt(name = "update-model", about = "Update a model in a source file")]
    UpdateModels {
        #[structopt(help = "", name = "json")]
        json: String,
    },
    #[structopt(name = "update-route", about = "Update a route in a source file")]
    UpdateRoutes {
        #[structopt(help = "", name = "json")]
        json: String,
    },
    #[structopt(name = "delete-model", about = "Delete a model in a source file")]
    DeleteModels {
        #[structopt(help = "", name = "name")]
        name: String,
    },
    #[structopt(name = "delete-route", about = "Delete a route in a source file")]
    DeleteRoutes {
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

    match Command::from_args() {
        Command::Error => {
            eprintln!("An example error to test things with");
            eprintln!("A second line");
            std::process::exit(1);
        }
        Command::ListModels { file } => list_models(file),
        Command::ListRoutes { file } => list_routes(file),
        Command::UpdateModels { json } => update_models(json),
        Command::UpdateRoutes { json } => update_routes(json),
        Command::DeleteModels { name } => delete_models(name),
        Command::DeleteRoutes { name } => delete_routes(name),
        _ => {
            eprintln!("Unsupported or unknown operation");
            std::process::exit(1);
        }
    }

    Ok(())
}

fn list_models(file: PathBuf) {
    let reply = r#"
        [{
            "name": "Pets",
            "vars" : []
        }]
    "#;
    println!("{}", reply);
}

fn list_routes(file: PathBuf) {
    let reply = r#"
        [{
            "method": "GET",
            "url_path": "/v1/pets",
            "response_type": "Pet",
            "error_type": "Error",
            "vars": []
        }]
    "#;
    println!("{}", reply);
}

fn update_models(json: String) {
    println!("json file")
}
fn update_routes(json: String) {
    println!("json file")
}
fn delete_models(name: String) {
    println!("Model successfully deleted")
}
fn delete_routes(name: String) {
    println!("Route successfully deleted")
}
