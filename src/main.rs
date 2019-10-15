mod visitors;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "--list-models", about = "Lists models in a source file")]
    ListModels {
        #[structopt(help = "", parse(from_os_str), name = "file")]
        file: PathBuf,
    },
    #[structopt(name = "--list-routes", about = "Lists routes in a source file")]
    ListRoutes {
        #[structopt(help = "", parse(from_os_str), name = "file")]
        file: PathBuf,
    },
    #[structopt(
        name = "--test-error",
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
        _ => (),
    }

    Ok(())
}

fn list_models(file: PathBuf) {
    let reply = r#"
        [{
            "name": "Pets"
        }]
    "#;
    println!("{}", reply);
}

fn list_routes(file: PathBuf) {
    let reply = r#"
        [{
            "name": "Pets"
        }]
    "#;
    println!("{}", reply);
}
