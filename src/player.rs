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
    active_line_entity: Option<Entity>,
    boost: bool,
    turn_points: Vec<Turn>,
    camera_mode: CameraMode,
    front_hitbox: Option<Entity>,
    lines: Vec<Line>,
}

#[derive(Debug, PartialEq)]
pub enum CameraMode {
    Follow,
    Top,
}

impl CameraMode {
    pub fn transform(&self) -> Transform {
        let mut transform = Transform::default();
        match self {
            CameraMode::Follow => {
                transform.translation = Vec3::new(-10., 2., 2.);
                transform.rotation = Quat::from_rotation_y(PI / -2.0);
            }
            CameraMode::Top => {
                transform.translation = Vec3::new(-11.0, 30., 2.);
                transform.rotation = Quat::from_rotation_x(PI / -2.0);
            }
        }
        transform
    }
}

impl Default for Player {
    fn default() -> Self {
        let pos = Vec3::new(1000.0, 0.3, 1000.0);
        let turn = Turn {
            pos,
            direction: Direction::default(),
            turn_direction: TurnDirection::Left,
        };
        Self {
            speed: 3.0,
            pos,
            camera_mode: CameraMode::Follow,
            direction: Direction::default(),
            entity: None,
            camera: None,
            line_entity: None,
            active_line_entity: None,
            boost: false,
            turn_points: vec![turn],
            front_hitbox: None,
            lines: vec![],
        }
    }
}

impl Player {
    pub fn switch_camera_mode(&mut self) {
        if self.camera_mode == CameraMode::Follow {
            self.camera_mode = CameraMode::Top;
        } else if self.camera_mode == CameraMode::Top {
            self.camera_mode = CameraMode::Follow;
        }
    }
    pub fn apply_camera_mode(&mut self, transforms: &mut Query<&mut Transform>) {
        if let Some(camera) = self.camera {
            if let Ok(mut transform) = transforms.get_mut(camera) {
                *transform = self.camera_mode.transform();
            };
        }
    }

    pub fn boost(&mut self) {
        self.boost = true;
    }
    pub fn stop_boost(&mut self) {
        self.boost = false;
    }

    pub fn lines(&self) -> Vec<Line> {
        self.lines.clone()
    }

    pub fn spawn(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let entity = commands
            .spawn(SceneBundle {
                transform: Transform {
                    translation: Vec3::new(self.pos.x, self.pos.y, self.pos.z),
                    rotation: Quat::from_rotation_y(PI / 2.0),
                    scale: Vec3::new(1., 1., 1.),
                },
                scene: asset_server.load("models/bike.glb#Scene0"),
                ..default()
            })
            .id();

        // cube
        let cube = commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                material: materials.add(Color::rgb(0.0, 0.8, 0.0).into()),
                transform: Transform::from_xyz(1., 0.0, 0.0),
                ..default()
            })
            .id();

        let camera = commands
            .spawn(Camera3dBundle {
                transform: self.camera_mode.transform(),
                ..default()
            })
            .id();

        commands.entity(entity).push_children(&[camera]);
        commands.entity(entity).push_children(&[cube]);
        self.entity = Some(entity);
        self.camera = Some(camera);
        self.front_hitbox = Some(cube);
    }

    pub fn hitbox_position(
        &mut self,
        transforms: &mut Query<&mut GlobalTransform>,
    ) -> Option<Vec3> {
        if let Some(entity) = self.front_hitbox {
            if let Ok(transform) = transforms.get_mut(entity) {
                return Some(transform.translation());
            }
        }
        None
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
            //offset side
            let os = 0.0;
            //offset mid
            let ob = 0.6;
            match self.direction {
                Direction::Left => {
                    transform.rotation = Quat::from_rotation_y(PI);
                    transform.translation = Vec3::new(self.pos.x - ob, self.pos.y, self.pos.z + os);
                }
                Direction::Right => {
                    transform.rotation = Quat::from_rotation_y(0.0);
                    transform.translation = Vec3::new(self.pos.x + ob, self.pos.y, self.pos.z - os);
                }
                Direction::Forward => {
                    transform.rotation = Quat::from_rotation_y(PI / 2.0);
                    transform.translation = Vec3::new(self.pos.x - os, self.pos.y, self.pos.z - ob);
                }
                Direction::Backward => {
                    transform.rotation = Quat::from_rotation_y(PI * 3.0 / 2.0);
                    transform.translation = Vec3::new(self.pos.x + os, self.pos.y, self.pos.z + ob);
                }
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

    pub fn get_all_vertices_and_indices(&mut self) -> (Vec<Vec3>, Vec<u32>) {
        let len = self.turn_points.len();

        let mut total_indices: Vec<u32> = vec![];
        let mut total_vertices: Vec<Vec3> = vec![];

        self.lines = vec![];

        for i in 0..len - 1 {
            let a_turn = self.turn_points[i];
            let b_turn = self.turn_points[i + 1];
            let l = Line::new(i, a_turn, b_turn, 0.0, 0.1, 0.5);
            self.lines.push(l.clone());

            total_vertices.extend(l.vertices);
            total_indices.extend(l.indices);
        }
        (total_vertices, total_indices)
    }

    pub fn draw_active_line(
        &mut self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let len = self.turn_points.len();
        // last turn
        let a_turn = self.turn_points[len - 1];

        let dir = match a_turn.turn_direction {
            TurnDirection::Left => a_turn.direction.turn_left(),
            TurnDirection::Right => a_turn.direction.turn_right(),
        };

        let l = Line::new(
            0,
            a_turn,
            Turn {
                turn_direction: a_turn.turn_direction,
                direction: dir,
                pos: self.pos,
            },
            0.0,
            0.1,
            0.5,
        );

        if let Some(entity) = self.active_line_entity {
            commands.entity(entity).despawn_recursive();
        }
        self.active_line_entity = Some(crate::draw_mesh(
            commands, meshes, materials, l.vertices, l.indices,
        ));
    }
}
