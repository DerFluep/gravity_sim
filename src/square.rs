use crate::particle::Particle;
use crate::vec2::Vec2;

#[derive(Clone, Copy)]
pub struct Square {
    min: Vec2,
    max: Vec2,
    size: f64,
}

impl Square {
    pub fn new(start: Vec2, size: f64) -> Self {
        let end = Vec2::new(start.x + size, start.y + size);
        Square {
            min: start,
            max: end,
            size,
        }
    }

    pub fn min(&self) -> Vec2 {
        self.min
    }

    pub fn max(&self) -> Vec2 {
        self.max
    }

    pub fn size(&self) -> f64 {
        self.size
    }

    pub fn bounding_box(particles: &[Particle]) -> Square {
        let mut min_x = f64::MAX;
        let mut max_x = -f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_y = -f64::MAX;

        particles.iter().for_each(|point| {
            if point.position.x < min_x {
                min_x = point.position.x;
            } else if point.position.x > max_x {
                max_x = point.position.x;
            }
            if point.position.y < min_y {
                min_y = point.position.y;
            } else if point.position.y > max_y {
                max_y = point.position.y;
            }
        });

        let delta_x = max_x - min_x;
        let delta_y = max_y - min_y;
        let mid_x = delta_x / 2.0 + min_x;
        let mid_y = delta_y / 2.0 + min_y;
        let max_length = if delta_x > delta_y { delta_x } else { delta_y };

        Square {
            min: Vec2::new(mid_x - max_length / 2.0, mid_y - max_length / 2.0),
            max: Vec2::new(mid_x + max_length / 2.0, mid_y + max_length / 2.0),
            size: max_length,
        }
    }

    pub fn center(&self) -> Vec2 {
        self.min + self.size / 2.0
    }

    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }
}
