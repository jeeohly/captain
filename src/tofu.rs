use bevy::{
    app::App,
    prelude::*,
    sprite::{collide_aabb::collide, MaterialMesh2dBundle},
};

pub const MAX_TOFU_COUNT: usize = 20;

pub const TOFU_SPEED: f32 = 400.0;

pub const TOFU_TOP_BOUNDARY: f32 = 2400.0;
pub const TOFU_LEFT_BOUNDARY: f32 = -3200.0;
pub const TOFU_RIGHT_BOUNDARY: f32 = 3200.0;

use crate::{AppState, Money, Player};
use rand::{rngs::SmallRng, Rng as _, SeedableRng};

struct Rng(SmallRng);
impl Default for Rng {
    fn default() -> Self {
        Self(SmallRng::from_entropy())
    }
}

#[derive(Default, Event)]
struct GameOverEvent;

#[derive(Component)]
pub struct Tofu {
    pub lifetime: Timer,
}

#[derive(Component, Default, Clone)]
struct Direction {
    x: f32,
    y: f32,
}

#[derive(Default)]
struct TimeSince(f32);

fn tofu_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut tofus: Query<(Entity, &mut Tofu)>,
    mut money: ResMut<Money>,
) {
    for (tofu_entity, mut tofu) in &mut tofus {
        tofu.lifetime.tick(time.delta());
        if tofu.lifetime.finished() {
            money.0 += 15.0;
            commands.entity(tofu_entity).despawn();
            info!("Won $15! Current Money: ${:?}", money.0);
        }
    }
}

fn normalize_direction(direction: &mut Direction) {
    let norm = (direction.x.powi(2) + direction.y.powi(2)).sqrt() + 1e-10;
    direction.x /= norm;
    direction.y /= norm;
}

fn gen_rand(rng: &mut Rng, min: f32, max: f32) -> f32 {
    rng.0.gen::<f32>() * (max - min) + min
}

fn check_collision(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    tofu_query: Query<(Entity, &Transform), With<Tofu>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_over_event: EventWriter<GameOverEvent>,
) {
    for (player_entity, player_transform) in player_query.iter() {
        for (_, tofu_transform) in tofu_query.iter() {
            let collision = collide(
                player_transform.translation,
                Vec2::new(64.0, 64.0),
                tofu_transform.translation,
                Vec2::new(64.0, 64.0),
            );
            if collision.is_some() {
                info!("HIT!");
                app_state.set(AppState::GameOver);
                commands.entity(player_entity).despawn();
                game_over_event.send_default();
                break;
            }
        }
    }
}

fn game_restarter(
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    mut query: Query<Entity, With<Tofu>>,
    mut time_since: Local<TimeSince>,
    keyboard_input: Res<Input<KeyCode>>,
    mut money_count: ResMut<Money>,
) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
    }
    if keyboard_input.pressed(KeyCode::Space) {
        app_state.set(AppState::GameStart);
        time_since.0 = 0.0;
        money_count.0 = 0.0;
    }
}

fn spawn_tofu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, (With<Player>, Without<Tofu>)>,
    mut query_tofu: Query<(&mut Direction, &mut Transform), With<Tofu>>,
    mut rng: Local<Rng>,
    time: Res<Time>,
    mut time_since: Local<TimeSince>,
) {
    let player_position = player.single().translation;
    time_since.0 += time.delta_seconds();
    let allow_new_spawn = if time_since.0 > 0.4 {
        time_since.0 = 0.0;
        true
    } else {
        false
    };
    for (mut direction, mut transform) in query_tofu.iter_mut() {
        let tofu_position = transform.translation;
        let new_direction = player_position - tofu_position;
        direction.x = new_direction.x;
        direction.y = new_direction.y;
        normalize_direction(&mut direction);
        transform.translation.x += TOFU_SPEED * time.delta_seconds() * direction.x;
        transform.translation.y += TOFU_SPEED * time.delta_seconds() * direction.y;
    }

    if query_tofu.iter().len() >= MAX_TOFU_COUNT || !allow_new_spawn {
        return;
    }

    let rand = rng.0.gen::<f32>();
    let x;
    let y;
    if rand > 0.66 {
        x = gen_rand(&mut rng, TOFU_LEFT_BOUNDARY, TOFU_RIGHT_BOUNDARY);
        y = TOFU_TOP_BOUNDARY;
    } else if rand > 0.33 {
        x = TOFU_LEFT_BOUNDARY;
        y = gen_rand(&mut rng, 0., TOFU_TOP_BOUNDARY);
    } else {
        x = TOFU_RIGHT_BOUNDARY;
        y = gen_rand(&mut rng, 0., TOFU_TOP_BOUNDARY);
    }

    let texture = asset_server.load("tofu_face.png");
    commands.spawn((
        Direction::default(),
        SpriteBundle {
            texture,
            transform: Transform {
                translation: Vec3::new(x, y, 0.0),
                ..Default::default()
            },
            ..default()
        },
        Tofu {
            lifetime: Timer::from_seconds(20.0, TimerMode::Once),
        },
    ));
}

pub fn setup_score_board(mut commands: Commands) {
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    ..Default::default()
                },
            ),
            TextSection::from_style(TextStyle {
                ..Default::default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
    );
}

pub fn update_scoreboard(money: Res<Money>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = money.0.to_string();
}

pub struct TofuPlugin;

impl Plugin for TofuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_event::<GameOverEvent>()
            .add_systems(Startup, setup_score_board)
            .add_systems(
                Update,
                check_collision
                    .before(spawn_tofu)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                spawn_tofu
                    .before(tofu_lifetime)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(Update, tofu_lifetime.run_if(in_state(AppState::InGame)))
            .add_systems(Update, update_scoreboard.run_if(in_state(AppState::InGame)))
            .add_systems(Update, game_restarter.run_if(in_state(AppState::GameOver)))
            .init_resource::<Money>();
    }
}
