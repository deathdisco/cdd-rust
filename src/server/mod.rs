use jsonrpc_ws_server::*;
use jsonrpc_ws_server::jsonrpc_core::*;
use jsonrpc_core::types::error::Error;
use cdd::*;

use crate::models::*;

fn rpc_error(message: &str) -> jsonrpc_core::types::error::Error {
    jsonrpc_core::types::error::Error{
        code: jsonrpc_core::types::error::ErrorCode::InternalError,
        message: message.into(),
        data: None,
    }
}

fn log(msg: String) {
    println!("{}", crate::util::truncate(msg, 128));
}

pub fn start() {
    let mut io = IoHandler::new();

    io.add_method("update", update);
    io.add_method("parse", |params| {
        println!("parse: {:?}", params);
        Ok(Value::String("hello".into()))
    });

	let server = ServerBuilder::new(io)
		.start(&"0.0.0.0:3030".parse().unwrap())
		.expect("Server must start with no issues");

	server.wait().unwrap()
}

/// update a rust code block with directives from an adt structure
fn update(params: jsonrpc_core::Params) -> std::result::Result<Value, Error> {
    log(format!("-> update: {:?}", params));

    let params:UpdateRequest = params.parse()?;
    let project:Project = crate::parser::parse_code_to_project(&params.code)
        .map_err(|e| rpc_error(&format!("{}", e)))?;

    return crate::generator::update(params.project, &params.code)
        .map(|code| serde_json::json!({"code": code}))
        .map_err(|e| rpc_error(&format!("{}", e)));
}