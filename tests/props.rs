//! Tests for entity and global props.

use bevy_ecs::prelude::*;
use bevy_mod_props::prelude::*;

/// Entity props and global props must not interfere with each other. As of
/// bevy 0.19 resources are components on singleton entities, so a single type
/// used as both a resource and a component would despawn other holders on
/// every insert. The `GlobalProps` wrapper exists to prevent exactly that.
#[test]
fn entity_and_global_props_are_independent() {
    let mut world = World::new();

    let frodo = world.spawn_empty().set_prop("name", "frodo").id();
    let sam = world.spawn_empty().set_prop("name", "sam").id();
    world.set_prop("name", "middle_earth");

    assert_eq!(world.entity(frodo).get_prop::<Estr>("name"), "frodo");
    assert_eq!(world.entity(sam).get_prop::<Estr>("name"), "sam");
    assert_eq!(world.get_prop::<Estr>("name"), "middle_earth");
}

#[test]
fn props_are_readable_through_queries() {
    let mut world = World::new();

    world.spawn(Props::new().with("health", 100.0));
    world.insert_resource(GlobalProps(Props::new().with("difficulty", 2.0)));

    let entity_props = world.query::<&Props>().single(&world).unwrap();
    assert_eq!(entity_props["health"], 100.0);

    let global_props = world.resource::<GlobalProps>();
    assert_eq!(global_props["difficulty"], 2.0);
}

#[test]
fn commands_write_props() {
    let mut world = World::new();

    let frodo = world.spawn_empty().id();
    let mut commands = world.commands();
    commands.entity(frodo).set_prop("has_ring", true);
    commands.set_prop("age_of_ring", 4867.0);
    world.flush();

    assert!(world.entity(frodo).get_prop::<bool>("has_ring"));
    assert_eq!(world.get_prop::<f32>("age_of_ring"), 4867.0);
}
