use crate::*;

pub struct AnimalIndividial {
    fitness: f32,
    chromosome: genetic_algorithm::Chromosome,
}

impl AnimalIndividial {
    pub fn from_animal(animal: &Animal) -> Self {
        Self {
            fitness: animal.satiation as f32,
            chromosome: 
        }
    }

    pub fn into_animal(self, rng: &mut dyn RngCore) -> Animal {}
}

impl genetic_algorithm::Individual for AnimalIndividial {
    fn create(chromosome: genetic_algorithm::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }

    fn chromosome(&self) -> &genetic_algorithm::Chromosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}
