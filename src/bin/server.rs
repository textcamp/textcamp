use std::sync::RwLock;

use actix::prelude::*;
use actix_files::Files;
use actix_web::{
    http, middleware, web, App, Error, HttpMessage, HttpRequest, HttpResponse, HttpServer, Result,
};
use actix_web_actors::ws;

use log::{info, trace};
use serde::Deserialize;

use textcamp::actors::*;
use textcamp::core::*;
use textcamp::templates;

const SESSION_COOKIE: &str = "session";

async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<RwLock<World>>,
) -> Result<HttpResponse, Error> {
    let world = data.into_inner();

    // check to see if we have the session token stored in the cookie.
    // if not, 401 the request.
    let session_token = match req.cookie(SESSION_COOKIE) {
        Some(cookie) => {
            trace!("🍪 ... found cookie!");
            cookie.value().to_owned()
        }
        None => {
            trace!("🍪 ... No cookie! 👎");
            return Ok(HttpResponse::Unauthorized().finish().into_body());
        }
    };

    // attempt to validate the session token
    let session_id = world
        .read()
        .unwrap()
        .authenticate_session(&session_token)
        .await;

    // check to see if the token returns an Identifier for the hero.
    // if so, open the websocket and continue
    // if not, 401 the request.
    match session_id {
        Some(identifier) => {
            trace!("🍪 ... found the session! 🎉");
            ws::start(Connection::new(world, identifier), &req, stream)
        }
        None => {
            trace!("🍪 ... No session! 👎");
            Ok(HttpResponse::Unauthorized().finish().into_body())
        }
    }
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

#[derive(Deserialize, Debug)]
struct OTPQuery {
    token: String,
}

async fn otp(query: web::Query<OTPQuery>, data: web::Data<RwLock<World>>) -> HttpResponse {
    let world = data.into_inner();
    let result = world
        .write()
        .unwrap()
        .authenticate_otp(query.token.clone())
        .await;

    match result {
        Some(session_token) => {
            // Sucessful OTP token exchange. Set the session cookie and continue on!
            // TODO: Figure out how to make this last 30 days and persist through browser restarts
            let cookie = http::Cookie::build(SESSION_COOKIE, session_token)
                .path("/")
                .secure(true)
                .finish();

            HttpResponse::Found()
                .header(http::header::LOCATION, "/?welcome")
                .cookie(cookie)
                .finish()
                .into_body()
        }
        None => {
            // Failed OTP token exchange!
            let redirect = "/?bad-otp";
            HttpResponse::Found()
                .header(http::header::LOCATION, redirect)
                .finish()
                .into_body()
        }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Print out the received ENV for debugging
    for (k, v) in std::env::vars() {
        info!("{} = {}", k, v);
    }

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
            .service(web::resource("/otp").route(web::get().to(otp)))
            .service(Files::new("/", "site").index_file("index.html"))
    })
    .bind(server_url)?
    .run()
    .await
}
