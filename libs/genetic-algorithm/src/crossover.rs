pub use self::uniform::*;

use crate::*;

mod uniform;
pub trait CrossOverMethod {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}
