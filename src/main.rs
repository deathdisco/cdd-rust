mod extractor;
mod generators;
mod models;
mod parser;
mod rustfmt;
mod server;
mod util;
mod visitors;
mod writers;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = "127.0.0.1:7779";
    println!("Starting server on {}...", server);
    server::start(server)
}
