use rand::RngCore;

use crate::neuron::Neuron;

#[derive(Clone)]
pub struct Layer {
    pub neurons: Vec<Neuron>,
}

impl Layer {
    pub fn new(neurons: Vec<Neuron>) -> Self {
        Self { neurons }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    pub fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(rng, input_size))
            .collect();

        Layer { neurons }
    }
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

            let layer = Layer::random(&mut rng, 3, 2);

            let actual_biases: Vec<f32> = layer.neurons.iter().map(|neuron| neuron.bias).collect();

            let actual_weights: Vec<&[f32]> = layer
                .neurons
                .iter()
                .map(|neuron| neuron.weights.as_slice())
                .collect();

            let expected_weights: Vec<&[f32]> = vec![
                &[0.67383957, 0.8181262, 0.26284897],
                &[-0.53516835, 0.069369674, -0.7648182],
            ];

            assert_relative_eq!(actual_biases.as_slice(), [-0.6255188, 0.5238807].as_slice());
            assert_relative_eq!(actual_weights.as_slice(), expected_weights.as_slice());
        }
    }

    mod propagate {
        use approx::assert_relative_eq;

        use super::*;

        #[test]
        fn test() {
            let neurons = vec![
                Neuron::new(0.0, vec![0.1, 0.2, 0.3]),
                Neuron::new(0.0, vec![0.4, 0.5, 0.6]),
            ];

            let layer = Layer::new(vec![neurons[0].clone(), neurons[1].clone()]);

            let inputs = &[0.5, 0.0, 0.5];

            let actual_propagate = layer.propagate(inputs.to_vec());
            let expected_propagate: Vec<f32> = neurons
                .iter()
                .map(|neuron| neuron.propagate(inputs))
                .collect();

            assert_relative_eq!(actual_propagate.as_slice(), expected_propagate.as_slice());
        }
    }
}
