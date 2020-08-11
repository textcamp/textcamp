use actix::prelude::{Actor, Addr, Context, Handler, Message, Supervised, SystemService};
use log::info;
use std::collections::HashMap;

use crate::actors::{ClientText, Connection};
use crate::core::{Identifier, Update};

#[derive(Default, Debug)]
pub struct Delivery {
    addresses: HashMap<Identifier, Addr<Connection>>,
}

impl Delivery {
    pub fn new() -> Self {
        let addresses = HashMap::new();
        Self { addresses }
    }
}

impl Actor for Delivery {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("📬 Delivery started ...");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("📬 Delivery stopped!");
    }
}

impl Supervised for Delivery {}

impl SystemService for Delivery {}

impl Handler<Deliver> for Delivery {
    type Result = ();
    fn handle(&mut self, msg: Deliver, _ctx: &mut Self::Context) {
        for update in msg.messages {
            let recipient = match self.addresses.get(&update.to) {
                Some(addr) => addr,
                None => continue, // if this is for an unregistered address, skip it.
            };

            // if the recipient is no longer connected, skip it.
            if !recipient.connected() {
                continue;
            }

            let output = serde_json::to_string(&update.message).unwrap();

            recipient.do_send(ClientText::new(output));
        }
    }
}

impl Handler<Register> for Delivery {
    type Result = ();
    fn handle(&mut self, msg: Register, _ctx: &mut Self::Context) {
        info!("📬 Adding recipient {:?}", msg.identifier);
        self.addresses.insert(msg.identifier, msg.addr);
    }
}

impl Handler<Unregister> for Delivery {
    type Result = ();
    fn handle(&mut self, msg: Unregister, _ctx: &mut Self::Context) {
        info!("📪 Removing recipient {:?}", msg.identifier);
        self.addresses.remove(&msg.identifier);
    }
}

#[derive(Debug)]
pub struct Deliver {
    messages: Vec<Update>,
}

impl Deliver {
    pub fn new(messages: Vec<Update>) -> Self {
        Self { messages }
    }
}

impl Message for Deliver {
    type Result = ();
}

#[derive(Debug)]
pub struct Register {
    identifier: Identifier,
    addr: Addr<Connection>,
}

impl Register {
    pub fn new(identifier: Identifier, addr: Addr<Connection>) -> Self {
        Self { identifier, addr }
    }
}

impl Message for Register {
    type Result = ();
}

#[derive(Debug)]
pub struct Unregister {
    identifier: Identifier,
}

impl Unregister {
    pub fn new(identifier: Identifier) -> Self {
        Self { identifier }
    }
}

impl Message for Unregister {
    type Result = ();
}
