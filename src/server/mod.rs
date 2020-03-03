use cdd::*;
use jsonrpc_core::types::error::Error;
use jsonrpc_ws_server::jsonrpc_core::*;
use jsonrpc_ws_server::*;

use crate::models::*;

fn rpc_error(message: &str) -> jsonrpc_core::types::error::Error {
    jsonrpc_core::types::error::Error {
        code: jsonrpc_core::types::error::ErrorCode::InternalError,
        message: message.into(),
        data: None,
    }
}

fn log(msg: String) {
    println!("{}", crate::util::truncate(msg, 128));
    // println!("{}", msg);
}

pub fn start(hostname: &str, port: i32) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut io = IoHandler::new();
    let server = format!("{}:{}", hostname, port);

    println!("Starting server on {}...", server);

    io.add_method("update", update);
    io.add_method("parse", parse);
    io.add_method("serialise", serialise);
    io.add_method("deserialise", deserialise);

    let server = ServerBuilder::new(io).start(&server.parse()?)?;

    Ok(server.wait().unwrap())
}

fn serialise(params: jsonrpc_core::Params) -> std::result::Result<Value, Error> {
    log(format!("-> serialise: {:?}", params));
    
    let params: SerialiseRequest = params.parse()?;
    log(format!("-> serialise.params: {:?}", params));

    let json_ast:String = crate::parser::parse_code_to_json(&params.code)
        .map_err(|e| rpc_error(&format!("{:?}", e)))?;
    
    log(format!("-> serialise.params.json_ast: {}", json_ast));

    // use serde_json::{Deserializer, Value};
    // let stream = Deserializer::from_str(&json_ast).into_iter::<Value>();

    // let json = serde_json::from_str(&json_ast)
    //     .map_err(|e| rpc_error(&format!("serde_json::from_str: {:?}", e)))?;

    // let response = serde_json::json!({ "ast": stream });
    let response = serde_json::from_str(&format!("{{\"ast\": {}}}", json_ast))
        .map_err(|e| rpc_error(&format!("serde_json::from_str: {:?}", e)))?;

    Ok(response)
}

fn deserialise(params: jsonrpc_core::Params) -> std::result::Result<Value, Error> {
    log(format!("-> deserialise: {:?}", params));
    
    let params: ParseRequest = params.parse()?;
    let code:String = crate::parser::parse_json_to_code(&params.code)
        .map_err(|e| rpc_error(&format!("{}", e)))?;

    let response = serde_json::json!({ "code": code });
    println!("<- response: {}", code);

    Ok(response)
}

/// parse a rust code block into an ast structure
fn parse(params: jsonrpc_core::Params) -> std::result::Result<Value, Error> {
    log(format!("-> parse: {:?}", params));

    let params: ParseRequest = params.parse()?;
    let project: Project = crate::parser::parse_code_to_project(&params.code)
        .map_err(|e| rpc_error(&format!("{}", e)))?;

    let response = serde_json::json!({ "project": project });
    println!("<- response: {}", response);

    Ok(response)
}

/// update a rust code block with directives from an adt structure
fn update(params: jsonrpc_core::Params) -> std::result::Result<Value, Error> {
    log(format!("<- request: {:?}", params));

    let params: UpdateRequest = params.parse()?;
    let project: Project = crate::parser::parse_code_to_project(&params.code)
        .map_err(|e| rpc_error(&format!("{}", e)))?;

    crate::generators::update(&params.project, &params.code)
        .map(|code| {
            let json = serde_json::json!({"code": code});
            log(format!("-> response: {:?}", json));
            json
        })
        .map_err(|e| rpc_error(&format!("{}", e)))
}
