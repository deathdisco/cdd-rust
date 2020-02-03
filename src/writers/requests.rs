use cdd::*;

const TEMPLATE:&str = r#"use actix_web::{web, HttpRequest, HttpResponse, Responder};
use crate::schema::pet::dsl::*;
use crate::models::*;
use crate::diesel::*;

pub(crate) fn root(_req: HttpRequest) -> impl Responder {
    format!("/")
}

pub(crate) fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        $ROUTES
    );
}

$METHODS
"#;

const METHOD_TEMPLATE:&str = r#"
fn $METHOD_NAME(req: HttpRequest) -> impl Responder {
    let connection = crate::establish_connection();
    let data = pet.load::<Pet>(&connection).unwrap();
    format!("{:?}", data)
}
"#;

pub fn print_requests(requests: Vec<Request>) -> String {
    TEMPLATE
        .replace("$ROUTES", &print_routes(&requests))
        .replace("$METHODS", &print_methods(&requests))
        .to_string()
}

pub fn print_routes(requests: &Vec<Request>) -> String {
    requests
        .into_iter()
        .map(request_to_route_call)
        .collect::<Vec<String>>()
        .join("\n\n")
}

pub fn print_methods(requests: &Vec<Request>) -> String {
    requests
        .into_iter()
        .map(request_to_string)
        .collect::<Vec<String>>()
        .join("\n\n")
}

fn request_to_route_call(request: &Request) -> String {
    let method = format!("{:?}", request.method).to_lowercase();

    format!("web::resource(\"{}\").route(web::{}().to({}))",
        request.path,
        method,
        request.name)
}

fn request_to_string(request: &Request) -> String {
    METHOD_TEMPLATE
        .replace("$METHOD_NAME", &request.name)
        .to_string()
}
