mod generator;
mod map;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};
use clap::Parser;
use actix_web::{web, App, HttpServer, Responder};
use actix_cors::Cors;

use generator::Generator;
use map::Map;

#[derive(Debug, Serialize, Deserialize)]
struct GenerationRequest {
    seed: String,
    width: u32,
    height: u32
}

async fn get_scene(item: web::Json<GenerationRequest>) -> impl Responder {
    let mut hasher = DefaultHasher::new();
    item.seed.hash(&mut hasher);
    let gen = Generator::new(hasher.finish());
    let map = Map::new(item.width, item.height);
    gen.generate(&map);
    web::Json(map)
}

/// Lynx map generator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port number
   port: u16
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(web::resource("/get_scene").route(web::get().to(get_scene)))
    })
    .bind(("127.0.0.1", args.port))?
    .run()
    .await
}