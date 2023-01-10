use bevy::prelude::Vec3;

use crate::{
    direction::Direction,
    player::{Turn, TurnDirection},
};

#[derive(Debug, Clone)]
pub struct EdgePoints {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    d: Vec3,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub index: usize,
    pub edge_points: EdgePoints,
    pub y_offset: f32,
    pub height: f32,
    pub vertices: Vec<Vec3>,
    pub indices: Vec<u32>,
}

impl Line {
    pub fn new(index: usize, a: Turn, b: Turn, y_offset: f32, width: f32, height: f32) -> Self {
        let edge_points = calculate_edge_points(&a, &b, width);
        Self {
            vertices: generate_vertices(&edge_points, y_offset, height),
            indices: generate_indices(index as u32),
            edge_points,
            y_offset,
            height,
            index,
        }
    }

    pub fn is_hit(&self, pos: &Vec3) {
        //bugged
        let a = self.edge_points.a;
        let b = self.edge_points.b;
        let c = self.edge_points.c;
        let d = self.edge_points.d;

        let xs = vec![a.x, b.x, c.x, d.x];
        let zs = vec![a.z, b.z, c.z, d.z];

        let min_x = xs.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let max_x = xs.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let min_z = zs.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let max_z = zs.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

        let min_px = pos.x - min_x;
        let min_pz = pos.z - min_z;

        let max_px = pos.x - max_x;
        let max_pz = pos.z - max_z;

        // buffer value
        let bv = 0.05;

        if min_px > -bv && min_pz > -bv && max_px < bv && max_pz < bv {
            println!("----\nHIT {:?}\n-----", std::time::Instant::now());
        }
    }
    // Generates square from 4 points without bottom square
}
fn generate_vertices(edge_points: &EdgePoints, y_offset: f32, height: f32) -> Vec<Vec3> {
    let y = y_offset;
    let h = height;
    let a = edge_points.a;
    let b = edge_points.b;
    let c = edge_points.c;
    let d = edge_points.d;

    vec![
        //A
        Vec3::new(a.x, y, a.z),     //0
        Vec3::new(a.x, y + h, a.z), //1
        //B
        Vec3::new(b.x, y, b.z),     //2
        Vec3::new(b.x, y + h, b.z), //3
        //C
        Vec3::new(c.x, y, c.z),     //4
        Vec3::new(c.x, y + h, c.z), //5
        //D
        Vec3::new(d.x, y, d.z),     //6
        Vec3::new(d.x, y + h, d.z), //7
    ]
}

pub fn generate_indices(index: u32) -> Vec<u32> {
    vec![
        // long side
        0, 1, 2, //
        3, 2, 1, //
        // long side
        6, 5, 4, //
        5, 6, 7, //
        // short side
        0, 4, 5, //
        5, 1, 0, //
        // short side
        7, 6, 2, //
        2, 3, 7, //
        //top
        5, 3, 1, //
        3, 5, 7, //
    ]
    .iter()
    // i * 8 since we have 8 vertices and we need to offset them
    .map(|x| x + index * 8)
    .collect()
}

fn calculate_edge_points(a_turn: &Turn, b_turn: &Turn, width: f32) -> EdgePoints {
    let mut a = a_turn.pos;
    let b = b_turn.pos;
    let mut c = a;
    let mut d = b;

    let w = width;

    if b_turn.turn_direction == TurnDirection::Right {
        // set points
        match b_turn.direction {
            Direction::Left => {
                c.x -= w;
                d.x -= w;
            }
            Direction::Right => {
                c.x += w;
                d.x += w;
            }
            Direction::Forward => {
                c.z -= w;
                d.z -= w;
            }
            Direction::Backward => {
                c.z += w;
                d.z += w;
            }
        };

        // fill gaps
        if a_turn.turn_direction == TurnDirection::Left {
            match b_turn.direction {
                Direction::Left => {
                    a.z -= w;
                    c.z -= w;
                }
                Direction::Right => {
                    a.z += w;
                    c.z += w;
                }
                Direction::Forward => {
                    a.x += w;
                    c.x += w;
                }
                Direction::Backward => {
                    a.x -= w;
                    c.x -= w;
                }
            };
        }
    } else {
        match b_turn.direction {
            Direction::Left => {
                c.x += w;
                d.x += w;
            }
            Direction::Right => {
                c.x -= w;
                d.x -= w;
            }
            Direction::Forward => {
                c.z += w;
                d.z += w;
            }
            Direction::Backward => {
                c.z -= w;
                d.z -= w;
            }
        };

        //fill gaps
        if a_turn.turn_direction == TurnDirection::Left {
            match b_turn.direction {
                Direction::Left => {
                    a.z += w;
                    c.z += w;
                }
                Direction::Right => {
                    a.z -= w;
                    c.z -= w;
                }
                Direction::Forward => {
                    a.x -= w;
                    c.x -= w;
                }
                Direction::Backward => {
                    a.x += w;
                    c.x += w;
                }
            };
        }
    }
    EdgePoints { a, b, c, d }
}
