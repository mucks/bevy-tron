use bevy::{
    prelude::*,
    render::{mesh, render_resource::PrimitiveTopology},
};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use player::Player;

mod direction;
mod player;

#[derive(Resource, Default)]
pub struct Game {
    pub player: Player,
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InfiniteGridPlugin)
        .add_startup_system(setup)
        .add_system(player_movement_system)
        .run();
}

fn draw_line(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let a = Vec3::new(0.0, 0.0, 0.0);
    let b = Vec3::new(2.0, 0.0, 0.0);
    let c = Vec3::new(0.0, 0.0, 1.0);
    let d = Vec3::new(2.0, 0.0, 1.0);

    // Positions of the vertices
    // See https://bevy-cheatbook.github.io/features/coords.html
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            //A -> B
            [a.x, 1., a.z],
            [a.x, 0., a.z],
            [b.x, 0., b.z],
            //
            [a.x, 1., a.z],
            [b.x, 0., b.z],
            [b.x, 1., b.z],
            // A -> C
            [a.x, 0., a.z],
            [a.x, 1., a.z],
            [-c.x, 0., -c.z],
            //
            [a.x, 1., a.z],
            [-c.x, 0., -c.z],
            [-c.x, 1., -c.z],
        ],
    );

    // In this example, normals and UVs don't matter,
    // so we just use the same value for all of them
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 12]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 12]);

    // A triangle using vertices 0, 2, and 1.
    // Note: order matters. [0, 1, 2] will be flipped upside down, and you won't see it from behind!
    mesh.set_indices(Some(mesh::Indices::U32(vec![
        0, 1, 2, //
        3, 4, 5, //
        6, 7, 8, //
        11, 10, 9,
    ])));

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
) {
    if keyboard_input.pressed(KeyCode::Comma) {
        game.player.boost();
    } else {
        game.player.stop_boost();
    }
    if keyboard_input.just_pressed(KeyCode::A) {
        game.player.turn_left();
    }
    if keyboard_input.just_pressed(KeyCode::E) {
        game.player.turn_right();
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
    draw_line(&mut commands, &mut meshes, &mut materials);

    commands.spawn(InfiniteGridBundle::default());
    //plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
