//! Defines bevy resources needed for Interpolation
use crate::timeline::InterpolationTimeline;
use bevy_ecs::component::Component;
use bevy_reflect::Reflect;
use core::cell::UnsafeCell;
use lightyear_replication::registry::ComponentError;
use lightyear_replication::registry::registry::ComponentRegistry;
use lightyear_serde::entity_map::EntityMap;

#[derive(Default, Debug, Reflect)]
pub struct InterpolatedEntityMap {
    // map from the confirmed entity to the interpolated entity
    // useful for despawning, as we won't have access to the Confirmed/Interpolated components anymore
    pub(crate) confirmed_to_interpolated: EntityMap,
}

#[derive(Component, Default)]
#[require(InterpolationTimeline)]
pub struct InterpolationManager {
    /// Map between confirmed and interpolated entities
    ///
    /// We wrap it into an UnsafeCell because the MapEntities trait requires a mutable reference to the EntityMap,
    /// but in our case calling map_entities will not mutate the map itself; by doing so we can improve the parallelism
    /// by avoiding a `ResMut<PredictionManager>` in our systems.
    pub(crate) interpolated_entity_map: UnsafeCell<InterpolatedEntityMap>,
}

// SAFETY: We never use UnsafeCell to mutate the interpolated_entity_map, so it's safe to send and sync
unsafe impl Send for InterpolationManager {}
unsafe impl Sync for InterpolationManager {}

impl InterpolationManager {
    pub fn new() -> Self {
        Self {
            interpolated_entity_map: Default::default(),
        }
    }

    /// Call MapEntities on the given component.
    ///
    /// Using this function only requires `&self` instead of `&mut self` (on the MapEntities trait), which is useful for parallelism
    pub(crate) fn map_entities<C: 'static>(
        &self,
        component: &mut C,
        component_registry: &ComponentRegistry,
    ) -> Result<(), ComponentError> {
        // SAFETY: `EntityMap` isn't mutated during `map_entities`
        unsafe {
            let entity_map = &mut *self.interpolated_entity_map.get();
            component_registry
                .map_entities::<C>(component, &mut entity_map.confirmed_to_interpolated)
        }
    }
}
