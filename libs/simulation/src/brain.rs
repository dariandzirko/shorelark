use crate::*;

pub struct Brain {
    pub(crate) nn: neural_network::Network,
}

impl Brain {
    pub fn random(rng: &mut dyn RngCore, eye: &Eye) -> Self {
        Self {
            nn: neural_network::Network::random(rng, &Self::topology(eye)),
        }
    }

    pub(crate) fn from_chromosome(chromosome: genetic_algorithm::Chromosome, eye: &Eye) -> Self {
        Self {
            nn: neural_network::Network::from_weights(&Self::topology(eye), chromosome),
        }
    }

    pub(crate) fn as_chromosome(&self) -> genetic_algorithm::Chromosome {
        self.nn.weights().collect()
    }

    fn topology(eye: &Eye) -> [neural_network::layer_topology::LayerTopology; 3] {
        [
            neural_network::layer_topology::LayerTopology {
                neurons: eye.cells(),
            },
            neural_network::layer_topology::LayerTopology {
                neurons: 2 * eye.cells(),
            },
            neural_network::layer_topology::LayerTopology { neurons: 2 },
        ]
    }
}
