use rand::prelude::*;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: simulation::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = simulation::Simulation::random(&mut rng);

        Self { rng, sim }
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        //Need to work on the deprecated feature
        JsValue::from_serde(&world).unwrap()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct World {
    pub animals: Vec<Animal>,
}

impl From<&simulation::World> for World {
    fn from(world: &simulation::World) -> Self {
        let animals = world.animals().iter().map(Animal::from).collect();

        Self { animals }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
}

impl From<&simulation::Animal> for Animal {
    fn from(animal: &simulation::Animal) -> Self {
        Self {
            x: animal.position().x,
            y: animal.position().y,
        }
    }
}
