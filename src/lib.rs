//! Provides utilities for stringly-typed data storage in bevy.
//!
//! # Features
//!
//! This crate includes:
//! + Key-value properties for entities and the world. See [`props`].
//! + Unique entity names and classes. See [`registry`].
//! + Arbitrary unidirectional links between entities. See [`links`].
//!
//! ```
//! # use bevy_ecs::prelude::*;
//! # use bevy_mod_props::prelude::*;
//! fn setup(mut commands: Commands) {
//!     let mut gandalf = commands.spawn_empty()
//!         .set_name("gandalf")
//!         .set_class("wizard")
//!         .set_prop("likes_elves", true)
//!         .id();
//!
//!     let mut bilbo = commands.spawn_empty()
//!         .set_name("bilbo")
//!         .set_class("hobbit")
//!         .set_link("talking_to", gandalf)
//!         .set_prop("has_ring", true)
//!         .set_prop("wearing_ring", false)
//!         .set_prop("health", 100.0)
//!         .set_prop("wearing", "elven_cloak");
//! }
//!
//! fn system(world: &mut World) -> Result {
//!     let bilbo = world.entity_mut_named("bilbo")?;
//!     if bilbo.get_prop::<Estr>("wearing") == "elven_cloak" {
//!         if let Some(entity_talked_to) = bilbo.follow_link("talking_to")
//!             && entity_talked_to.get_prop("likes_elves")
//!         {
//!             // have the npc say something about bilbo's cloak here
//!         }
//!     }
//!     Ok(())
//! }
//! ```

pub mod links;
pub mod props;
pub mod registry;

#[doc(hidden)]
pub mod prelude {
    pub use crate::links::*;
    pub use crate::props::*;
    pub use crate::registry::*;
    pub use estr::Estr;
}
