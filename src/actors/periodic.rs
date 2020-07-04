use actix::prelude::*;
use log::{debug, info};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::actors::*;
use crate::core::{Update, World};

// How long we wait (system time) between ticks
const TICK_INTERVAL: Duration = Duration::from_secs(5);

// How long we wait (system time) between melee rounds
const MELEE_INTERVAL: Duration = Duration::from_millis(1000);

#[derive(Debug)]
pub struct Periodic {
    world: Arc<RwLock<World>>,
}

impl Periodic {
    pub fn new(world: Arc<RwLock<World>>) -> Self {
        Self { world }
    }

    pub fn send_updates(&self, updates: Vec<Update>) {
        if !updates.is_empty() {
            let delivery = Delivery::from_registry();
            delivery.do_send(Deliver::new(updates));
        }
    }
}

impl Actor for Periodic {
    type Context = Context<Periodic>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("🚀 World starting!");

        ctx.run_interval(TICK_INTERVAL, |act, _ctx| {
            let started = Instant::now();

            let updates = act.world.write().unwrap().tick();
            act.send_updates(updates);

            let world = act.world.read().unwrap();
            let clock = world.clock();

            info!(
                "⏱  Tick {:?} - Time: {}:{}, Date: {}/{}/{} - Mobs: {}, Spaces: {} ({:?})",
                clock.tick,
                clock.hour(),
                clock.minute(),
                clock.day_of_month(),
                clock.month(),
                clock.year(),
                world.mobs.len(),
                world.spaces.len(),
                started.elapsed()
            );
        });

        ctx.run_interval(MELEE_INTERVAL, |act, _ctx| {
            let started = Instant::now();

            let updates = act.world.write().unwrap().melee();
            let memo_length = updates.len();
            act.send_updates(updates);

            if memo_length > 0 {
                debug!("⚔️  {:?}", started.elapsed());
            }
        });
    }
}
