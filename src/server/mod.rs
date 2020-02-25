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
    io.add_method("parse", parse);

	let server = ServerBuilder::new(io)
		.start(&"0.0.0.0:7779".parse().unwrap())
		.expect("Server must start with no issues");

	server.wait().unwrap()
}

/// parse a rust code block into an ast structure
fn parse(params: jsonrpc_core::Params) -> std::result::Result<Value, Error> {
    log(format!("-> parse: {:?}", params));

    let params:ParseRequest = params.parse()?;
    let project:Project = crate::parser::parse_code_to_project(&params.code)
        .map_err(|e| rpc_error(&format!("{}", e)))?;
    

    return Ok(serde_json::json!({"project": project}))
}

/// update a rust code block with directives from an adt structure
fn update(params: jsonrpc_core::Params) -> std::result::Result<Value, Error> {
    log(format!("-> update: {:?}", params));

    let params:UpdateRequest = params.parse()?;
    let project:Project = crate::parser::parse_code_to_project(&params.code)
        .map_err(|e| rpc_error(&format!("{}", e)))?;

    return crate::generators::update(&params.project, &params.code)
        .map(|code| serde_json::json!({"code": code}))
        .map_err(|e| rpc_error(&format!("{}", e)));
}