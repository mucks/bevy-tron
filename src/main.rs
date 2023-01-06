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

pub fn draw_line(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    a: Vec3,
    b: Vec3,
    c: Vec3,
    d: Vec3,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    //y
    let y = 0.0;
    //height
    let h = 0.5;

    // Positions of the vertices
    // See https://bevy-cheatbook.github.io/features/coords.html
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            //A
            [a.x, y, a.z],     //0
            [a.x, y + h, a.z], //1
            //B
            [b.x, y, b.z],     //2
            [b.x, y + h, b.z], //3
            //C
            [c.x, y, c.z],     //4
            [c.x, y + h, c.z], //5
            //D
            [d.x, y, d.z],     //6
            [d.x, y + h, d.z], //7
        ],
    );

    // In this example, normals and UVs don't matter,
    // so we just use the same value for all of them
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 8]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 8]);

    // A triangle using vertices 0, 2, and 1.
    // Note: order matters. [0, 1, 2] will be flipped upside down, and you won't see it from behind!
    mesh.set_indices(Some(mesh::Indices::U32(vec![
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
