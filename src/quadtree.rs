use crate::particle::Particle;
use crate::square::Square;
use crate::vec2::Vec2;

use macroquad::prelude::*;

enum NodeType {
    Empty,
    Leaf(usize),
    Internal([usize; 4]),
}

struct Node {
    bounds: Square,
    mass: f64,
    center_of_mass: Vec2,
    particle_index: usize,
    kind: NodeType,
}

pub struct Quadtree {
    nodes: Vec<Node>,
}

impl Quadtree {
    pub fn create(particles: &[Particle]) -> Self {
        let bounds = Square::bounding_box(particles);
        let root = Node {
            bounds,
            mass: 0.0,
            center_of_mass: bounds.center(),
            particle_index: 0,
            kind: NodeType::Empty,
        };

        let mut tree = Quadtree { nodes: vec![root] };

        let root_index = 0;

        for (n, _) in particles.iter().enumerate() {
            tree.insert(root_index, n, particles);
        }

        // TODO: iterate from the bottom up and calculate mass and center of mass for every Internal
        // type

        tree
    }

    pub fn insert(&mut self, node_index: usize, particle_index: usize, particles: &[Particle]) {
        match self.nodes[node_index].kind {
            NodeType::Empty => {
                self.nodes[node_index].mass = particles[particle_index].mass;
                self.nodes[node_index].center_of_mass = particles[particle_index].position;
                self.nodes[node_index].particle_index = particle_index;
                self.nodes[node_index].kind = NodeType::Leaf(particle_index);
            }
            NodeType::Leaf(current_particle) => {
                // create 4 child squares and push 4 new nodes with those squares into the tree
                let bounds = self.nodes[node_index].bounds;
                let bounds_min = bounds.min();
                let bounds_size = bounds.size();

                let sw = Square::new(bounds_min, bounds_size / 2.0);
                let se = Square::new(
                    bounds_min + Vec2::new(bounds_size / 2.0, 0.0),
                    bounds_size / 2.0,
                );
                let nw = Square::new(
                    bounds_min + Vec2::new(0.0, bounds_size / 2.0),
                    bounds_size / 2.0,
                );
                let ne = Square::new(bounds_min + bounds_size / 2.0, bounds_size / 2.0);

                let child_index = self.nodes.len();
                self.nodes.push(Node {
                    bounds: sw,
                    mass: 0.0,
                    center_of_mass: sw.center(),
                    particle_index: 0,
                    kind: NodeType::Empty,
                });
                self.nodes.push(Node {
                    bounds: se,
                    mass: 0.0,
                    center_of_mass: se.center(),
                    particle_index: 0,
                    kind: NodeType::Empty,
                });
                self.nodes.push(Node {
                    bounds: nw,
                    mass: 0.0,
                    center_of_mass: nw.center(),
                    particle_index: 0,
                    kind: NodeType::Empty,
                });
                self.nodes.push(Node {
                    bounds: ne,
                    mass: 0.0,
                    center_of_mass: ne.center(),
                    particle_index: 0,
                    kind: NodeType::Empty,
                });

                // select the appropriate child node for the current particle
                if sw.contains(particles[current_particle].position) {
                    self.insert(child_index, current_particle, particles);
                } else if se.contains(particles[current_particle].position) {
                    self.insert(child_index + 1, current_particle, particles);
                } else if nw.contains(particles[current_particle].position) {
                    self.insert(child_index + 2, current_particle, particles);
                } else if ne.contains(particles[current_particle].position) {
                    self.insert(child_index + 3, current_particle, particles);
                }

                // select the appropriate child node for the new particle
                if sw.contains(particles[particle_index].position) {
                    self.insert(child_index, particle_index, particles);
                } else if se.contains(particles[particle_index].position) {
                    self.insert(child_index + 1, particle_index, particles);
                } else if nw.contains(particles[particle_index].position) {
                    self.insert(child_index + 2, particle_index, particles);
                } else if ne.contains(particles[particle_index].position) {
                    self.insert(child_index + 3, particle_index, particles);
                }

                // Update the current node type
                self.nodes[node_index].kind = NodeType::Internal([
                    child_index,
                    child_index + 1,
                    child_index + 2,
                    child_index + 3,
                ]);
            }
            NodeType::Internal(childs) => {
                for child in childs.iter() {
                    if self.nodes[*child]
                        .bounds
                        .contains(particles[particle_index].position)
                    {
                        self.insert(*child, particle_index, particles);
                        break;
                    }
                }
            }
        }
    }

    pub fn draw(&self, offset_x: f32, offset_y: f32) {
        for node in self.nodes.iter() {
            let square = node.bounds;
            draw_rectangle_lines(
                square.min().x as f32 + offset_x,
                square.min().y as f32 + offset_y,
                square.size() as f32,
                square.size() as f32,
                2.0,
                GREEN,
            );
        }
    }
}
