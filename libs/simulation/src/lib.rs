use nalgebra::{Point2, Rotation2, Similarity2};
use rand::{distributions::Alphanumeric, Rng, RngCore};

pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }
}
pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..40).map(|_| Animal::random(rng)).collect();
        let foods = (0..40).map(|_| Food::random(rng)).collect();

        Self { animals, foods }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}

pub struct Animal {
    position: Point2<f32>,
    rotation: Rotation2<f32>,
    speed: f32,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> Rotation2<f32> {
        self.rotation
    }
}

pub struct Food {
    position: nalgebra::Point2<f32>,
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }
}
