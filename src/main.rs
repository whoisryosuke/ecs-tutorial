use crate::ecs::world::World;
mod ecs;

// Our "Components"
struct Health(i32);
struct Name(&'static str);

fn main() {
    // Create a new "World" to store our Components and Entities
    let mut world = World::new();

    // Create a new Entity inside the World
    let ryo = world.new_entity();
    world.add_component_to_entity(ryo, Health(100));
    world.add_component_to_entity(ryo, Name("Ryo"));

    // Grab all components from world that match the type we provide
    let mut all_health = world
        .borrow_component_vec::<Health>()
        .expect("No health found");
    let mut all_names = world
        .borrow_component_vec::<Name>()
        .expect("No names found");

    // Combine the two loops
    // Since each one is a Vec, and the order represents entity ids
    // we can merge safely without thinking too much
    let combined = all_health.iter_mut().zip(all_names.iter_mut());
    let filtered = combined.filter_map(|(health, name)| Some((health.as_mut()?, name.as_mut()?)));

    for (health, name) in filtered {
        // Our structs above are tuples, so we access the first (and only) value by using `.0` (aka the 0 index)
        println!("{} has {} health", name.0, health.0);
    }
}
