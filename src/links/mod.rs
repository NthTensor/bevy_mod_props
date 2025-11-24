//! Adds simple links between entities.
//!
//! ```
//! # use bevy_ecs::prelude::*;
//! # use bevy_mod_props::prelude::*;
//! #
//! # let mut links = Links::default();
//! # let troll = Entity::PLACEHOLDER;
//! # let goblin = Entity::PLACEHOLDER;
//! links.add("looking_at", troll);
//! links.add("looking_at", goblin);
//!
//! for look_target in links.list("looking_at") {
//!     // ...
//! }
//! ```

use bevy_ecs::{
    component::Component,
    entity::{Entity, EntityHashSet},
};
use ustr::{Ustr, UstrMap};

mod ext;
pub use ext::*;

/// Stores links between entities.
///
/// Links are somewhat similar to relations, with a few notable limitations:
///
/// 1. They are identified by string names, rather than types.
/// 2. They are unidirectional.
///
/// To create one-to-one links, use [`set`][Links::set] and [`get`][Links::get]. You can also create
/// many-to-one or many-to-many links using [`add`][Links::add] and [`list`][Links::list].
#[derive(Component, Default)]
pub struct Links {
    links: UstrMap<EntityHashSet>,
}

impl Links {
    /// Sets a link to a specific entity. The previous value of this link will be overwritten.
    pub fn set(&mut self, name: impl Into<Ustr>, target: Entity) {
        let link = self.links.entry(name.into()).or_default();
        link.clear();
        link.insert(target);
    }

    /// Adds a link to a specific entity. The same link can point to multiple entities.
    pub fn add(&mut self, name: impl Into<Ustr>, target: Entity) {
        let link = self.links.entry(name.into()).or_default();
        link.insert(target);
    }

    /// Removes an entity from a link.
    pub fn remove(&mut self, name: impl Into<Ustr>, target: Entity) {
        let link = self.links.entry(name.into()).or_default();
        link.remove(&target);
    }

    /// Clears the value of a link.
    pub fn clear(&mut self, name: impl Into<Ustr>) {
        let link = self.links.entry(name.into()).or_default();
        link.clear();
    }

    /// Returns true if the entity is linked under this name.
    pub fn is_linked(&self, name: impl Into<Ustr>, entity: Entity) -> bool {
        if let Some(link) = self.links.get(&name.into()) {
            link.contains(&entity)
        } else {
            false
        }
    }

    /// Returns the linked entity. If the link points to multiple entities,
    /// any of them may be returned (which is explicetly left undefined).
    pub fn get(&self, name: impl Into<Ustr>) -> Option<Entity> {
        self.links
            .get(&name.into())
            .and_then(|entities| entities.iter().next())
            .copied()
    }

    /// Returns all linked entities. If the link points to multiple entities,
    /// all will be returned.
    pub fn list(&self, name: impl Into<Ustr>) -> EntityHashSet {
        self.links
            .get(&name.into())
            .cloned()
            .unwrap_or(EntityHashSet::new())
    }
}
