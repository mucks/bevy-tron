use std::f32::consts::PI;

use crate::direction::Direction;
use bevy::{
    prelude::*,
    render::{mesh, render_resource::PrimitiveTopology},
};

#[derive(Debug)]
pub struct Player {
    speed: f32,
    pos: Vec2,
    direction: Direction,
    entity: Option<Entity>,
    camera: Option<Entity>,
    y: f32,
    boost: bool,
    turn_points: Vec<Vec2>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 1.0,
            pos: Vec2::new(0.0, 0.0),
            direction: Direction::default(),
            entity: None,
            camera: None,
            y: 0.5,
            boost: false,
            turn_points: Vec::new(),
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
                    translation: Vec3::new(self.pos.x, self.y, self.pos.y),
                    rotation: Quat::from_rotation_y(PI / 2.0),
                    scale: Vec3::splat(0.5),
                },
                scene: asset_server.load("models/bike.glb#Scene0"),
                ..default()
            })
            .id();
        let camera = commands
            .spawn(Camera3dBundle {
                transform: Transform {
                    translation: Vec3::new(-9.0, 2.4, -0.8),
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
            Direction::Forward => self.pos.y -= self.speed * delta_speed,
            Direction::Backward => self.pos.y += self.speed * delta_speed,
        }

        if let Ok(mut transform) = transforms.get_mut(self.entity.unwrap()) {
            transform.translation = Vec3::new(self.pos.x, self.y, self.pos.y);
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
        self.turn_points.push(self.pos);
    }
    pub fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
        self.turn_points.push(self.pos);
    }

    pub fn draw_knots(&self) {}
}
