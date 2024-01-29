use bevy::{
    app::App,
    prelude::*,
    render::camera::ScalingMode,
    sprite::{collide_aabb::collide, MaterialMesh2dBundle},
};

use tofu::TofuPlugin;

mod tofu;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
enum AppState {
    #[default]
    GameStart,
    InGame,
    GameOver,
}

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

#[derive(Resource, Default)]
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

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 6400.0,
        min_height: 4800.0,
    };
    commands.spawn(camera);
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut app_state: ResMut<NextState<AppState>>,
    input: Res<Input<KeyCode>>,
) {
    if input.pressed(KeyCode::Space) {
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
        app_state.set(AppState::InGame);
    }
}
pub fn start_page(mut commands: Commands) {
    commands.spawn(
        TextBundle::from_section(
            "press space to start\nwasd/arrow_keys to move",
            TextStyle {
                ..Default::default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Percent(50.0),
            left: Val::Percent(50.0),
            ..default()
        }),
    );
}

fn main() {
    App::new()
        .add_state::<AppState>()
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
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_player.run_if(in_state(AppState::GameStart)))
        .add_systems(
            Update,
            character_movement.run_if(in_state(AppState::InGame)),
        )
        .add_plugins(TofuPlugin)
        .run();
}
