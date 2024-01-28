use bevy::prelude::*;

use crate::{Money, Player};

#[derive(Component)]
pub struct Tofu {
    pub lifetime: Timer,
}

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
            info!("Tofu sold for $15! Current Money: ${:?}", money.0);
        }
    }
}

fn spawn_tofu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player.single();
    if money.0 >= 10.0 {
        money.0 -= 10.0;
        info!("Spent $10 on a tofu, remaining money: ${:?}", money.0);

        let texture = asset_server.load("tofu_face.png");
        commands.spawn((
            SpriteBundle {
                texture,
                transform: *player_transform,
                ..default()
            },
            Tofu {
                lifetime: Timer::from_seconds(2.0, TimerMode::Once),
            },
        ));
    }
}

pub struct TofuPlugin;

impl Plugin for TofuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_tofu, tofu_lifetime));
    }
}
