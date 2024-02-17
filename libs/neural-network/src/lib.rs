use layer::*;
use layer_topology::LayerTopology;
use rand::{Rng, RngCore};

mod layer;
pub mod layer_topology;
mod neuron;

pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn weights(&self) -> Vec<f32> {
        use std::iter::once;

        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .copied()
            .collect()
    }

    pub fn from_weights(layers: &[LayerTopology], weights: impl IntoIterator<Item = f32>) -> Self {}
}

#[cfg(test)]
mod tests {
    use super::*;

    mod random {

        use super::*;
        use approx::assert_relative_eq;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        #[test]
        fn test() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());

            let network = Network::random(
                &mut rng,
                &[
                    LayerTopology { neurons: 3 },
                    LayerTopology { neurons: 2 },
                    LayerTopology { neurons: 1 },
                ],
            );

            assert_eq!(network.layers.len(), 2);
            assert_eq!(network.layers[0].neurons.len(), 2);

            assert_relative_eq!(network.layers[0].neurons[0].bias, -0.6255188);
            assert_relative_eq!(
                network.layers[0].neurons[0].weights.as_slice(),
                &[0.67383957, 0.8181262, 0.26284897].as_slice()
            );

            assert_relative_eq!(network.layers[0].neurons[1].bias, 0.5238807);
            assert_relative_eq!(
                network.layers[0].neurons[0].weights.as_slice(),
                &[0.67383957, 0.8181262, 0.26284897].as_slice()
            );

            assert_eq!(network.layers[1].neurons.len(), 1);
            assert_relative_eq!(
                network.layers[1].neurons[0].weights.as_slice(),
                &[-0.48879617, -0.19277132].as_slice()
            );
        }
    }

    mod propagate {
        use approx::assert_relative_eq;

        use self::neuron::Neuron;

        use super::*;

        #[test]
        fn test() {
            let layers = vec![
                Layer::new(vec![
                    Neuron::new(0.0, vec![-0.5, -0.4, -0.3]),
                    Neuron::new(0.0, vec![-0.2, -0.1, 0.0]),
                ]),
                Layer::new(vec![Neuron::new(0.0, vec![-0.5, 0.5])]),
            ];

            let network = Network::new(vec![layers[0].clone(), layers[1].clone()]);

            let inputs = vec![0.5, 0.6, 0.7];

            let actual = network.propagate(inputs.clone());
            let expected = layers[1].propagate(layers[0].propagate(inputs.clone()));

            assert_relative_eq!(actual.as_slice(), expected.as_slice());
        }
    }
}
