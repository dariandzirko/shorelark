pub use self::{animal::*, brain::*, eye::*, food::*, world::*};

mod animal;
mod animal_individual;
mod brain;
mod eye;
mod food;
mod world;

use self::animal_individual::*;
use genetic_algorithm::{self, GeneticAlgorithm, Individual};
use nalgebra::{Point2, Rotation2};
use neural_network;
use rand::{Rng, RngCore};

use std::f32::consts::FRAC_PI_2;

const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.005;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;

const GENERATION_LENGTH: usize = 2500;

pub struct Simulation {
    world: World,
    ga: GeneticAlgorithm<genetic_algorithm::RouletteWheelSelection>,
    age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng);
        let ga = GeneticAlgorithm::new(
            genetic_algorithm::RouletteWheelSelection,
            genetic_algorithm::UniformCrossover,
            genetic_algorithm::GaussianMutation::new(0.01, 0.3),
        );
        Self { world, ga, age: 0 }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<genetic_algorithm::Statistics> {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();

        self.age += 1;

        if self.age > GENERATION_LENGTH {
            Some(self.evolve(rng))
        } else {
            None
        }
    }

    pub fn train(&mut self, rng: &mut dyn RngCore) -> genetic_algorithm::Statistics {
        loop {
            if let Some(summary) = self.step(rng) {
                return summary;
            }
        }
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * nalgebra::Vector2::new(0.0, animal.speed);

            animal.position.x = nalgebra::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = nalgebra::wrap(animal.position.y, 0.0, 1.0);
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = nalgebra::distance(&animal.position, &food.position);

                if distance <= 0.01 {
                    animal.satiation += 1;
                    food.position = rng.gen();
                }
            }
        }
    }

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            let vision =
                animal
                    .eye
                    .process_vision(animal.position, animal.rotation, &self.world.foods);
            let response = animal.brain.nn.propagate(vision);

            let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);

            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            animal.speed = (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);
            animal.rotation = nalgebra::Rotation2::new(animal.rotation.angle() + rotation);
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) -> genetic_algorithm::Statistics {
        self.age = 0;

        let current_population: Vec<_> = self
            .world
            .animals
            .iter()
            .map(|animal| AnimalIndividial::from_animal(animal))
            .collect();

        let (evolved_population, stats) = self.ga.evolve(rng, &current_population);

        self.world.animals = evolved_population
            .into_iter()
            .map(|individual| individual.into_animal(rng))
            .collect();

        for food in &mut self.world.foods {
            food.position = rng.gen()
        }

        stats
    }
}
