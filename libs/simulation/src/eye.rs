use std::f32::consts::*;

use crate::*;

const FOV_RANGE: f32 = 0.25;
const FOV_ANGLE: f32 = PI + FRAC_PI_4;
const CELLS: usize = 9;

pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    cells: usize,
}

impl Eye {
    fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        Self {
            fov_angle,
            fov_range,
            cells,
        }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(
        &self,
        posiition: nalgebra::Point2<f32>,
        rotation: nalgebra::Rotation2<f32>,
        foods: &[Food],
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells()];

        for food in foods {
            let vec = food.position - posiition;

            let dist = vec.norm();

            if dist >= self.fov_range {
                continue;
            }

            let mut angle =
                nalgebra::Rotation2::rotation_between(&nalgebra::Vector2::y(), &vec).angle();

            angle = nalgebra::wrap(angle - rotation.angle(), -PI, PI);

            if angle.abs() > self.fov_angle / 2.0 {
                continue;
            }

            angle += self.fov_angle / 2.0;

            let cell = (angle / self.fov_angle) * self.cells() as f32;

            let cell = (cell as usize).min(cells.len() - 1);

            let energy = (self.fov_range - dist) / self.fov_range;

            cells[cell] = energy;
        }
        cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self {
            fov_range: FOV_RANGE,
            fov_angle: FOV_ANGLE,
            cells: CELLS,
        }
    }
}
