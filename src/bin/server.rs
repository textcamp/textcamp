use std::sync::RwLock;

use actix::prelude::*;
use actix_files::Files;
use actix_web::{http, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;

use serde::Deserialize;

use textcamp::actors::*;
use textcamp::core::*;
use textcamp::templates;

async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<RwLock<World>>,
) -> Result<HttpResponse, Error> {
    let world = data.into_inner();
    ws::start(Connection::new(world), &req, stream)
}

#[derive(Deserialize, Debug)]
struct AuthForm {
    email: String,
}

async fn start_auth(form: web::Form<AuthForm>, data: web::Data<RwLock<World>>) -> HttpResponse {
    let world = data.into_inner();
    world
        .write()
        .unwrap()
        .authentication
        .start_auth(&form.email)
        .await;

    let redirect = "/?check-email";
    HttpResponse::Found()
        .header(http::header::LOCATION, redirect)
        .finish()
        .into_body()
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
    templates::bootstrap(&world_root, &mut world);

    // Prepare the world for sharing across connections
    let world_data = web::Data::new(RwLock::new(world));

    // Periodic timer controls ticks and melee
    Periodic::new(world_data.clone().into_inner()).start();

    HttpServer::new(move || {
        App::new()
            .app_data(world_data.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
            .service(web::resource("/start-auth").route(web::post().to(start_auth)))
            .service(Files::new("/", "site").index_file("index.html"))
    })
    .bind(server_url)?
    .run()
    .await
}
