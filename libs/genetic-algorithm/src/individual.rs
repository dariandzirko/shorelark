use crate::*;

pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
    fn create(chromosome: Chromosome) -> Self;
}

#[cfg(test)]
pub struct TestIndividual {
    fitness: f32,
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn fitness(&self) -> f32 {
        self.fitness
    }

    fn chromosome(&self) -> &Chromosome {
        panic!("not supported for TestIndividual")
    }

    fn create(chromosome: Chromosome) -> Self {
        panic!("not supported for TestIndidivual")
    }
}