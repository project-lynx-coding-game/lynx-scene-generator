mod generator;
mod map;

#[cfg(feature = "rendering-images")]
mod render;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};
use clap::Parser;
use actix_web::{web, App, HttpServer, HttpResponse};
use actix_cors::Cors;

use generator::Generator;
use map::Map;

#[cfg(feature = "rendering-images")]
use render::render_image;

#[derive(Debug, Serialize, Deserialize)]
struct GenerationRequest {
    seed: String,
    width: u32,
    height: u32
}

async fn get_scene(item: web::Json<GenerationRequest>) -> HttpResponse {
    let mut hasher = DefaultHasher::new();
    item.seed.hash(&mut hasher);
    let gen = Generator::new(hasher.finish());
    let mut map = Map::new(item.width, item.height);
    gen.generate(&mut map);

    #[cfg(feature = "rendering-images")]
    render_image(&map, &item.seed);

    HttpResponse::Ok().body(map.to_string())
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
            .service(web::resource("/get_scene").route(web::post().to(get_scene)))
    })
    .bind(("127.0.0.1", args.port))?
    .run()
    .await
}