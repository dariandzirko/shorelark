use crate::*;

pub struct Animal {
    pub(crate) position: Point2<f32>,
    pub(crate) rotation: Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: Eye,
    pub(crate) brain: neural_network::Network,
    pub(crate) satiation: usize,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();

        let brain = neural_network::Network::random(
            rng,
            &[
                neural_network::layer_topology::LayerTopology {
                    neurons: eye.cells(),
                },
                neural_network::layer_topology::LayerTopology {
                    neurons: eye.cells() * 2,
                },
                neural_network::layer_topology::LayerTopology { neurons: 2 },
            ],
        );

        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
            satiation: 0,
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> Rotation2<f32> {
        self.rotation
    }
}
