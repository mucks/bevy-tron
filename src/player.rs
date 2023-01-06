use std::f32::consts::PI;

use crate::{direction::Direction, line::Line};
use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TurnDirection {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct Turn {
    pub pos: Vec3,
    pub direction: Direction,
    pub turn_direction: TurnDirection,
}

#[derive(Debug)]
pub struct Player {
    speed: f32,
    pos: Vec3,
    direction: Direction,
    entity: Option<Entity>,
    camera: Option<Entity>,
    line_entity: Option<Entity>,
    boost: bool,
    turn_points: Vec<Turn>,
}

impl Default for Player {
    fn default() -> Self {
        let pos = Vec3::new(1000.0, 0.5, 1000.0);
        Self {
            speed: 2.0,
            pos,
            direction: Direction::default(),
            entity: None,
            camera: None,
            line_entity: None,
            boost: false,
            turn_points: vec![Turn {
                pos,
                direction: Direction::default(),
                turn_direction: TurnDirection::Left,
            }],
        }
    }
}

impl Player {
    pub fn boost(&mut self) {
        self.boost = true;
    }
    pub fn stop_boost(&mut self) {
        self.boost = false;
    }
    pub fn spawn(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let entity = commands
            .spawn(SceneBundle {
                transform: Transform {
                    translation: Vec3::new(self.pos.x, self.pos.y, self.pos.z),
                    rotation: Quat::from_rotation_y(PI / 2.0),
                    scale: Vec3::new(0.5, 0.5, 0.5),
                },
                scene: asset_server.load("models/bike.glb#Scene0"),
                ..default()
            })
            .id();
        let camera = commands
            .spawn(Camera3dBundle {
                transform: Transform {
                    translation: Vec3::new(-11.0, 2.4, -0.8),
                    rotation: Quat::from_rotation_y(PI / -2.0),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                },
                ..default()
            })
            .id();
        commands.entity(entity).push_children(&[camera]);
        self.entity = Some(entity);
        self.camera = Some(camera);
    }

    pub fn drive(&mut self, transforms: &mut Query<&mut Transform>, delta: f32) {
        let mut delta_speed = self.speed * delta;

        if self.boost {
            delta_speed *= 2.0;
        }

        match self.direction {
            Direction::Left => self.pos.x -= self.speed * delta_speed,
            Direction::Right => self.pos.x += self.speed * delta_speed,
            Direction::Forward => self.pos.z -= self.speed * delta_speed,
            Direction::Backward => self.pos.z += self.speed * delta_speed,
        }

        if let Ok(mut transform) = transforms.get_mut(self.entity.unwrap()) {
            transform.translation = Vec3::new(self.pos.x, self.pos.y, self.pos.z);
            match self.direction {
                Direction::Left => transform.rotation = Quat::from_rotation_y(PI),
                Direction::Right => transform.rotation = Quat::from_rotation_y(0.0),
                Direction::Forward => transform.rotation = Quat::from_rotation_y(PI / 2.0),
                Direction::Backward => transform.rotation = Quat::from_rotation_y(PI * 3.0 / 2.0),
            }
        }
    }

    pub fn turn_left(&mut self) {
        self.direction = self.direction.turn_left();
        self.turn_points.push({
            Turn {
                pos: self.pos,
                direction: self.direction,
                turn_direction: TurnDirection::Left,
            }
        });
    }
    pub fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
        self.turn_points.push(Turn {
            pos: self.pos,
            direction: self.direction,
            turn_direction: TurnDirection::Right,
        });
    }

    pub fn draw_all_lines(
        &mut self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let (vertices, indices) = self.get_all_vertices_and_indices();

        //remove old line mesh
        if let Some(entity) = self.line_entity {
            commands.entity(entity).despawn_recursive();
        }
        //draw new line mesh
        self.line_entity = Some(crate::draw_mesh(
            commands, meshes, materials, vertices, indices,
        ));
    }

    pub fn get_all_vertices_and_indices(&self) -> (Vec<Vec3>, Vec<u32>) {
        let len = self.turn_points.len();

        let mut total_indices: Vec<u32> = vec![];
        let mut total_vertices: Vec<Vec3> = vec![];

        for i in 0..len - 1 {
            let a_turn = self.turn_points[i];
            let b_turn = self.turn_points[i + 1];
            let l = Line::new(i, a_turn, b_turn, 0.0, 0.1, 0.5);

            total_vertices.extend(l.vertices);
            total_indices.extend(l.indices);
        }
        (total_vertices, total_indices)
    }

    pub fn draw_line(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let len = self.turn_points.len();
        // last turn
        let a_turn = self.turn_points[len - 2];
        // current turn
        let b_turn = self.turn_points[len - 1];

        let l = Line::new(0, a_turn, b_turn, 0.0, 0.1, 0.5);

        crate::draw_mesh(commands, meshes, materials, l.vertices, l.indices);
    }
}
