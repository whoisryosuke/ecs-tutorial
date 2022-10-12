use std::{
    any::Any,
    cell::{RefCell, RefMut},
};

/// Top-level "world", contains children "entities"
pub struct World {
    /// Number of entities in system
    pub entities_count: usize,
    /// Sets of components in the system.
    /// Each "element" represents a single component assigned and the entities it's attached to.
    // We store this in a Vec to iterate over.
    // We also use Box because our entities will by dynamic in size
    pub component_sets: Vec<Box<dyn ComponentSet>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            component_sets: Vec::new(),
        }
    }

    /// Create a new entity in the world.
    /// Returns the new entity ID, used for referencing within the world/system.
    pub fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;

        // Loop over entities
        for component_set in self.component_sets.iter_mut() {
            // For every set of components we insert a `None`
            // to signify this Entity doesn't have that component
            component_set.push_none();
        }
        self.entities_count += 1;

        entity_id
    }

    /// Add a Component (aka struct/enum) to an Entity
    pub fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        // Loop over all component containers and check for existing component set
        for component_set in self.component_sets.iter_mut() {
            // Check if this component type matches an existing component
            if let Some(component_set) = component_set
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
            {
                // Add the component to the component set
                // and attach the Component to the specific Entity
                // We use `Some()` here because it can also be `None`
                component_set.get_mut()[entity] = Some(component);
                return;
            }
        }

        // If component type doesn't exist in system
        // create new component set to contain it
        let mut new_component_set: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities_count);

        // All existing entities don't have this component so we label it `None`
        for _ in 0..self.entities_count {
            new_component_set.push(None);
        }

        // "Attach" the Entity the Component
        new_component_set[entity] = Some(component);
        self.component_sets
            .push(Box::new(RefCell::new(new_component_set)));
    }

    /// Borrow a mutable instance of a certain type of component
    /// (e.g. `borrow_component_vec::<Health>()`)
    pub fn borrow_component_vec<ComponentType: 'static>(
        &self,
    ) -> Option<RefMut<Vec<Option<ComponentType>>>> {
        for component_vec in self.component_sets.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
            {
                // Here we use `borrow_mut`.
                // If this `RefCell` is already borrowed from this will panic.
                return Some(component_vec.borrow_mut());
            }
        }
        None
    }
}

/// Trait used to make structs/enums "Components Sets"
/// (aka 1 component that gets applied to some/all entities).
/// This is used internally, no need to use this on Components.
pub trait ComponentSet {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn push_none(&mut self);
}

impl<T: 'static> ComponentSet for RefCell<Vec<Option<T>>> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    // This handles filling the component set with `None` types if Entity has no components
    fn push_none(&mut self) {
        self.get_mut().push(None)
    }
}
