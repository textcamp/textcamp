use actix::prelude::*;
use actix_files::Files;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;
use std::sync::RwLock;

use textcamp::actors::*;
use textcamp::core::*;
use textcamp::templates;

async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<RwLock<World>>,
) -> Result<HttpResponse, Error> {
    let world = data.into_inner();

    // TODO: Login!
    let character_id = world.read().unwrap().create_hero();
    ws::start(Connection::new(world, character_id), &req, stream)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let server_url = std::env::var("SERVER_URL").expect("SERVER_URL must be set");
    let world_root = std::env::var("WORLD_ROOT").unwrap_or_else(|_| "./world".to_owned());

    // Shared state between our actors
    let mut world = World::new();

    // load world data from templates
    bootstrap(&world_root, &mut world);

    // Prepare the world for sharing across connections
    let world_data = web::Data::new(RwLock::new(world));

    // Periodic timer controls ticks and melee
    Periodic::new(world_data.clone().into_inner()).start();

    HttpServer::new(move || {
        App::new()
            .app_data(world_data.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
            .service(Files::new("/", "site").index_file("index.html"))
    })
    .bind(server_url)?
    .run()
    .await
}

fn bootstrap(world_root: &str, world: &mut World) {
    templates::bootstrap(world_root, world);
}
