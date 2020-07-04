mod importer;

pub mod item;
pub mod mob;
pub mod space;

use crate::core::World;
use importer::*;

pub fn bootstrap(path: &str, world: &mut World) {
    Importer::load_dir(path)
        .into_iter()
        .for_each(|t| match t.is_a() {
            Kind::Space => space::Injector::from(t).inject(world),
            Kind::Mob => mob::Injector::from(t).inject(world),
            Kind::Item => item::Injector::from(t).inject(world),
        });
}
