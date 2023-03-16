use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::{
        mesh,
        render_resource::{PrimitiveTopology, Texture},
    },
};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin};
use bevy_inspector_egui::{egui::TextureHandle, WorldInspectorPlugin};
use player::Player;

mod direction;
mod line;
mod player;

#[derive(Resource, Default)]
pub struct Game {
    pub player: Player,
}

fn main() {
    //println!("Hello, world!");
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InfiniteGridPlugin)
        .add_startup_system(setup)
        .add_system(player_movement_system)
        .run();
}

pub fn draw_mesh(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    vertices: Vec<Vec3>,
    indices: Vec<u32>,
) -> Entity {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vertices
            .iter()
            .map(|v| [v.x, v.y, v.z])
            .collect::<Vec<[f32; 3]>>(),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; vertices.len()]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; vertices.len()]);

    mesh.set_indices(Some(mesh::Indices::U32(indices)));

    let material = StandardMaterial {
        base_color: Color::rgb(0.9, 0.1, 0.1),
        unlit: false,
        ..Default::default()
    };

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(material),
            ..default()
        })
        .id()
}

// Player movement system
fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    mut global_transforms: Query<&mut GlobalTransform>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keyboard_input.pressed(KeyCode::Comma) {
        game.player.boost();
    } else {
        game.player.stop_boost();
    }

    if keyboard_input.just_pressed(KeyCode::Y) {
        game.player.switch_camera_mode();
        game.player.apply_camera_mode(&mut transforms);
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        game.player.turn_left();
        game.player
            .draw_all_lines(&mut commands, &mut meshes, &mut materials);
    }
    if keyboard_input.just_pressed(KeyCode::E) {
        game.player.turn_right();
        game.player
            .draw_all_lines(&mut commands, &mut meshes, &mut materials);
    }
    game.player.drive(&mut transforms, time.delta_seconds());
    game.player
        .draw_active_line(&mut commands, &mut meshes, &mut materials);

    // check if player hits his own line
    if let Some(pos) = game.player.hitbox_position(&mut global_transforms) {
        for line in game.player.lines() {
            if line.is_hit(&pos) {
                game.player.handle_got_hit(&mut commands);
                println!("HIT!");
            }
        }
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    game.player
        .spawn(&mut commands, &asset_server, &mut meshes, &mut materials);

    commands.spawn(InfiniteGridBundle::default());

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            range: 1000.0,
            ..default()
        },
        transform: Transform::from_xyz(1000.0, 8.0, 1000.0),
        ..default()
    });
}
