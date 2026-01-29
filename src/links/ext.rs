use bevy_ecs::{
    entity::{Entity, EntityHashMap, EntityHashSet},
    system::EntityCommands,
    world::{EntityMut, EntityRef, EntityWorldMut},
};
use estr::Estr;

use super::Links;

// -----------------------------------------------------------------------------
// Immutable links access

pub trait LinksExt {
    fn get_linked(&self, name: impl Into<Estr>) -> Option<Entity>;

    fn list_linked(&self, name: impl Into<Estr>) -> EntityHashSet;

    fn is_linked(&self, name: impl Into<Estr>, target: Entity) -> bool;
}

impl<'w> LinksExt for EntityRef<'w> {
    fn get_linked(&self, name: impl Into<Estr>) -> Option<Entity> {
        self.get::<Links>()?.get(name)
    }

    fn list_linked(&self, name: impl Into<Estr>) -> EntityHashSet {
        match self.get::<Links>() {
            Some(links) => links.list(name),
            None => EntityHashSet::default(),
        }
    }

    fn is_linked(&self, name: impl Into<Estr>, target: Entity) -> bool {
        match self.get::<Links>() {
            Some(links) => links.is_linked(name, target),
            None => false,
        }
    }
}

impl<'w> LinksExt for EntityWorldMut<'w> {
    fn get_linked(&self, name: impl Into<Estr>) -> Option<Entity> {
        self.get::<Links>()?.get(name)
    }

    fn list_linked(&self, name: impl Into<Estr>) -> EntityHashSet {
        match self.get::<Links>() {
            Some(links) => links.list(name),
            None => EntityHashSet::default(),
        }
    }

    fn is_linked(&self, name: impl Into<Estr>, target: Entity) -> bool {
        match self.get::<Links>() {
            Some(links) => links.is_linked(name, target),
            None => false,
        }
    }
}

// -----------------------------------------------------------------------------
// Link commands

pub trait LinksCommandsExt {
    fn set_link(&mut self, name: impl Into<Estr>, target: Entity) -> &mut Self;

    fn add_link(&mut self, name: impl Into<Estr>, target: Entity) -> &mut Self;

    fn remove_link(&mut self, name: impl Into<Estr>, target: Entity) -> &mut Self;

    fn clear_links(&mut self, name: impl Into<Estr>) -> &mut Self;
}

impl<'w> LinksCommandsExt for EntityWorldMut<'w> {
    fn set_link(&mut self, name: impl Into<Estr>, target: Entity) -> &mut Self {
        self.entry::<Links>()
            .or_default()
            .into_mut()
            .set(name, target);
        self
    }

    fn add_link(&mut self, name: impl Into<Estr>, target: Entity) -> &mut Self {
        self.entry::<Links>()
            .or_default()
            .into_mut()
            .add(name, target);
        self
    }

    fn remove_link(&mut self, name: impl Into<Estr>, target: Entity) -> &mut Self {
        self.entry::<Links>()
            .or_default()
            .into_mut()
            .remove(name, target);
        self
    }

    fn clear_links(&mut self, name: impl Into<Estr>) -> &mut Self {
        self.entry::<Links>().or_default().into_mut().clear(name);
        self
    }
}

impl<'a> LinksCommandsExt for EntityCommands<'a> {
    fn set_link(&mut self, name: impl Into<Estr>, target: Entity) -> &mut Self {
        let name = name.into();
        self.queue(move |mut entity: EntityWorldMut| {
            entity.set_link(name, target);
        })
    }

    fn add_link(&mut self, name: impl Into<Estr>, target: Entity) -> &mut Self {
        let name = name.into();
        self.queue(move |mut entity: EntityWorldMut| {
            entity.add_link(name, target);
        })
    }

    fn remove_link(&mut self, name: impl Into<Estr>, target: Entity) -> &mut Self {
        let name = name.into();
        self.queue(move |mut entity: EntityWorldMut| {
            entity.remove_link(name, target);
        })
    }

    fn clear_links(&mut self, name: impl Into<Estr>) -> &mut Self {
        let name = name.into();
        self.queue(move |mut entity: EntityWorldMut| {
            entity.clear_links(name);
        })
    }
}

// -----------------------------------------------------------------------------
// Link following

pub trait LinksFollowExt<'w>: Sized {
    fn follow_link(self, name: impl Into<Estr>) -> Option<Self>;

    fn explore_link(self, name: impl Into<Estr>) -> EntityHashMap<EntityMut<'w>>;
}

impl<'w> LinksFollowExt<'w> for EntityWorldMut<'w> {
    fn follow_link(self, name: impl Into<Estr>) -> Option<Self> {
        let target = self.get_linked(name)?;
        let world = self.into_world_mut();
        world.get_entity_mut(target).ok()
    }

    fn explore_link(self, name: impl Into<Estr>) -> EntityHashMap<EntityMut<'w>> {
        let targets = self.list_linked(name);
        let world = self.into_world_mut();
        match world.get_entity_mut(&targets) {
            Ok(entities) => entities,
            Err(_) => EntityHashMap::default(),
        }
    }
}
