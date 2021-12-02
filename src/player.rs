use crate::*;

struct Player {
    seconds_falling: f32,
    is_falling: bool,
    is_jumping: bool,
    seconds_jumping: f32,
}
impl Default for Player {
    fn default() -> Self {
        Self {
            seconds_falling: 0.,
            is_falling: false,
            is_jumping: false,
            seconds_jumping: 0.,
        }
    }
}
struct GravityTimer(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self,  app: &mut AppBuilder){
        app.add_startup_system(spawn_player.system())
            .add_system(player_gravity.system())
            .add_system(player_jump.system())
            .add_system(player_collide_with_objects.system())
            .add_system(player_respawn.system())
            .insert_resource(GravityTimer(Timer::from_seconds(0.01, true)));
    }
}

fn spawn_player(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    //player
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1., 0.8, 0.).into()),
            sprite: Sprite::new(Vec2::new(32., 32.)),
            transform: Transform {
                translation: Vec3::new(0., 0., 1.).into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player::default());
}
fn player_gravity(
    time: Res<Time>,
    game_state: Res<GameState>,
    mut timer: ResMut<GravityTimer>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    if let Ok((mut transform, mut player)) = query.single_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            if game_state.0 == GameStates::GameActive {
                if player.is_falling {
                    transform.translation.y += -GRAVITY * player.seconds_falling;
                    player.seconds_falling += 0.01;
                } else if player.is_jumping {
                    transform.translation.y += GRAVITY;
                    if player.seconds_jumping < 0.05 {
                        player.seconds_jumping += 0.01;
                    } else {
                        player.seconds_jumping = 0.;
                        player.is_jumping = false;
                        player.is_falling = true;
                    }
                } else if game_state.0 == GameStates::PreGame {
                    transform.translation.x = 0.;
                    transform.translation.y = 0.;
                }
            }
        }
    }
}
fn player_jump(
        input: Res<Input<KeyCode>>,
        mouse: Res<Input<MouseButton>>,
        game_state: Res<GameState>,
        ready: Res<InputReady>,
        mut query: Query<&mut Player>,
    ) {
        if let Ok(mut player) = query.single_mut() {
            if game_state.0 != GameStates::GameOver && ready.0 && (input.pressed(KeyCode::Space) || mouse.pressed(MouseButton::Left)) {
                player.is_jumping = true;
                player.seconds_jumping = 0.;
                player.seconds_falling = 0.;
                player.is_falling = false;
            }
        }
}
fn player_respawn(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, mut game_state: ResMut<GameState>, mut query: Query<(Entity, With<Player>)>){

    if game_state.0 == GameStates::Reset {
        if let Ok((entity, _)) = query.single_mut() { 
            commands.entity(entity).despawn();
            spawn_player(commands, materials);
        } 
        game_state.0 = GameStates::PreGame;
    }

}
fn player_collide_with_objects(
        mut player_query: Query<(&Transform, &mut Player, &Sprite)>,
        mut ground_query: Query<(&Transform, &Sprite, With<Collider>)>,
        mut game_state: ResMut<GameState>,
    ) {
        if let Ok((player_transform, mut player, player_sprite)) = player_query.single_mut() {
            for (ground_transform, ground_sprite, _) in ground_query.iter_mut() {
                let collision = collide(
                    player_transform.translation,
                    player_sprite.size,
                    ground_transform.translation,
                    ground_sprite.size,
                );
                if collision.is_some() && game_state.0 == GameStates::GameActive {
                    player.is_falling = false;
                    player.seconds_falling = 0.;
                    game_state.0 = GameStates::GameOver;
                }
            }
        }
    }



