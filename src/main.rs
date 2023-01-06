use bevy::{
    prelude::*,
    render::{mesh, render_resource::PrimitiveTopology},
};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
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
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vertices
            .iter()
            .map(|v| [v.x, v.y, v.z])
            .collect::<Vec<[f32; 3]>>(),
    );

    // In this example, normals and UVs don't matter,
    // so we just use the same value for all of them
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 8]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 8]);

    // A triangle using vertices 0, 2, and 1.
    // Note: order matters. [0, 1, 2] will be flipped upside down, and you won't see it from behind!
    mesh.set_indices(Some(mesh::Indices::U32(indices)));

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0.9, 0., 0.).into()),
        ..default()
    });
}

// Player movement system
fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keyboard_input.pressed(KeyCode::Comma) {
        game.player.boost();
    } else {
        game.player.stop_boost();
    }
    if keyboard_input.just_pressed(KeyCode::A) {
        game.player.turn_left();
        game.player
            .draw_line(&mut commands, &mut meshes, &mut materials);
    }
    if keyboard_input.just_pressed(KeyCode::E) {
        game.player.turn_right();
        game.player
            .draw_line(&mut commands, &mut meshes, &mut materials);
    }

    game.player.drive(&mut transforms, time.delta_seconds());
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    game.player.spawn(&mut commands, &asset_server);

    commands.spawn(InfiniteGridBundle::default());
    //plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(1000., 8.0, 1000.),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(1000., 8.0, 1000.),
        ..default()
    });
}
