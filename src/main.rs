use bevy::{prelude::*, render::camera::ScalingMode};
use tofu::TofuPlugin;

mod tofu;

#[derive(Component, Reflect)]
pub struct Health {
    max: f32,
    current: f32,
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub health: Health,
}

#[derive(Resource)]
pub struct Money(pub f32);

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
            transform.translation.y += player.speed * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
            transform.translation.y -= player.speed * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
            transform.translation.x += player.speed * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
            transform.translation.x -= player.speed * time.delta_seconds();
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 6400.0,
        min_height: 4800.0,
    };
    commands.spawn(camera);

    let texture = asset_server.load("captain_face.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player {
            speed: 500.0,
            health: Health {
                max: 10.0,
                current: 2.0,
            },
        },
    ));
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "captainn".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .insert_resource(Money(100.0))
        .add_plugins(TofuPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .run();
}
