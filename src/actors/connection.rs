use actix::prelude::*;
use actix_web_actors::ws;
use log::{debug, info, trace};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::actors::*;
use crate::core::*;

// The heartbeat pings the websocket connection to ensure it stays alive
const HEARTBEAT_SEC: Duration = Duration::from_secs(30);

// How long to wait to close the connection if we haven't received any updates
// from a client (including heartbeat ping/pongs)
const CLIENT_TIMEOUT: Duration = Duration::from_secs(60);

// How often we send time (world clock) updates to the client
const TIME_UPDATES: Duration = Duration::from_secs(10);

/// Connection represents the interface between the player's websocket connection
/// and the World. It maintains the connection, parses commands, sends messages
/// to the player.
#[derive(Clone, Debug)]
pub struct Connection {
    /// represents the identifier of the character controlled by this connection
    identifier: Identifier,

    /// used to maintain the websocket connection
    ws_heartbeat: Instant,

    /// shared instance of the world
    world: Arc<RwLock<World>>,
}

impl Connection {
    pub fn new(world: Arc<RwLock<World>>, identifier: Identifier) -> Self {
        Connection {
            identifier,
            ws_heartbeat: Instant::now(),
            world,
        }
    }

    async fn send_command(&self, input: String) {
        match Phrase::from(&input) {
            Some(phrase) => {
                let action = Command::new(&self.identifier, phrase);
                let updates = self.world.write().unwrap().command(action).await;
                // not all updates are for this connection, so we send them over to the delivery actor
                let delivery = Delivery::from_registry();
                delivery.do_send(Deliver::new(updates));
            }
            None => debug!("Received empty message."),
        }
    }
}

// TODO: Drop for Connection, to clean up characters and message delivery for dead connections
impl Actor for Connection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("🔌✅ New connection!");

        // start the client websocket heartbeat
        ctx.run_interval(HEARTBEAT_SEC, |act, ctx| {
            if act.ws_heartbeat.elapsed() > CLIENT_TIMEOUT {
                info!("🔌 Timed out, disconnecting!");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });

        // start sending time updates to the client
        ctx.run_interval(TIME_UPDATES, |act, ctx| {
            let date_time: DateTime = act.world.read().unwrap().clock().clone().into();
            ctx.text(serde_json::to_string(&update::Wrapper::Time(date_time)).unwrap());
        });

        // register this connection for delivery
        let delivery = Delivery::from_registry();
        delivery.do_send(Register::new(self.identifier.clone(), ctx.address()));
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("🔌🚫 Disconnected!");
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Connection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        self.ws_heartbeat = Instant::now();
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                debug!("❤️  Pinged!");
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                debug!("❤️  Ponged!");
            }
            Ok(ws::Message::Text(text)) => {
                trace!("Received {}", text);
                let async_self = self.clone();
                ctx.wait(actix::fut::wrap_future(async move {
                    async_self.send_command(text).await
                }));
            }
            Ok(ws::Message::Close(reason)) => {
                debug!("Connection closed by client.");
                ctx.close(reason);
            }
            Ok(unknown) => {
                debug!("Unhandled websocket message type: {:?}", unknown);
            }
            _ => (),
        }
    }
}

impl Handler<ClientText> for Connection {
    type Result = ();
    fn handle(&mut self, msg: ClientText, ctx: &mut Self::Context) {
        ctx.text(msg.message);
    }
}

/// Contains a raw string to be sent out the websocket connection
#[derive(Clone, Debug)]
pub struct ClientText {
    pub message: String,
}

impl ClientText {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Message for ClientText {
    type Result = ();
}
