//! The machine plate: the family API standard, checked against `IsomeshPlugin`.
//!
//! Names come from `System::name()`, which is the full type path only when
//! Bevy's `debug` feature is on; the dev-dependency on `bevy` turns it on, and
//! the test fails loudly (empty prefix match) rather than passing vacuously if
//! it is ever off: it asserts it saw at least one `bevy_isomesh::` system.

use bevy_app::{App, TaskPoolPlugin};
use bevy_asset::AssetPlugin;
use bevy_ecs::reflect::{AppTypeRegistry, ReflectComponent, ReflectResource};
use bevy_ecs::schedule::{Schedules, SystemSet};
use bevy_isomesh::{
    ChunkMesh, Extractor, IsomeshPlugin, IsomeshSystems, NeedsRemesh, VoxelChunk, VoxelVolume,
};
use bevy_reflect::{GetTypeRegistration, TypeRegistry};

const CRATE: &str = "bevy_isomesh::";

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((TaskPoolPlugin::default(), AssetPlugin::default()))
        .add_plugins(IsomeshPlugin);
    app.update();
    app
}

/// Every system of the crate must be in one of the crate's root-set variants.
#[test]
fn every_system_is_in_the_root_set() {
    let app = app();
    let roots = [IsomeshSystems.intern()];
    let schedules = app.world().resource::<Schedules>();
    let mut seen = 0;
    let mut outside = Vec::new();
    for (label, schedule) in schedules.iter() {
        let Ok(systems) = schedule.systems() else {
            continue;
        };
        for (key, system) in systems {
            let name = system.name().to_string();
            if !name.starts_with(CRATE) {
                continue;
            }
            seen += 1;
            let in_root = roots.iter().any(|root| {
                schedule
                    .graph()
                    .systems_in_set(*root)
                    .is_ok_and(|set| set.contains(&key))
            });
            if !in_root {
                outside.push(format!("{name} in {label:?}"));
            }
        }
    }
    assert!(
        seen > 0,
        "no `{CRATE}` systems seen: is Bevy's `debug` feature on?"
    );
    assert!(
        outside.is_empty(),
        "systems outside IsomeshSystems: {outside:#?}"
    );
}

fn assert_component<T: GetTypeRegistration>(registry: &TypeRegistry) {
    let name = std::any::type_name::<T>();
    let registration = registry
        .get(std::any::TypeId::of::<T>())
        .unwrap_or_else(|| panic!("{name} is not registered by IsomeshPlugin"));
    assert!(
        registration.data::<ReflectComponent>().is_some(),
        "{name} has no ReflectComponent"
    );
}

/// Every public component is reflected and registered.
#[test]
fn components_are_reflected() {
    let app = app();
    let registry = app.world().resource::<AppTypeRegistry>().read();
    assert_component::<VoxelVolume>(&registry);
    assert_component::<VoxelChunk>(&registry);
    assert_component::<NeedsRemesh>(&registry);
    assert_component::<ChunkMesh>(&registry);
    // Not a component: registered so a volume's extractor is inspectable.
    assert!(registry.get(std::any::TypeId::of::<Extractor>()).is_some());
}

/// The crate has no settings resources (`MeshBudget` and `MeshStats` are
/// runtime state and are not reflected as resources).
#[test]
fn no_settings_resources() {
    let app = app();
    let registry = app.world().resource::<AppTypeRegistry>().read();
    let resources = registry
        .iter()
        .filter(|r| r.data::<ReflectResource>().is_some())
        .filter(|r| r.type_info().type_path().starts_with(CRATE))
        .map(|r| r.type_info().type_path().to_owned())
        .collect::<Vec<_>>();
    assert!(
        resources.is_empty(),
        "unexpected reflected resources: {resources:?}"
    );
}
