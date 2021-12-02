use crate::*;

struct PipeSpawnTimer(Timer);
pub enum Pipe {
    Top,
    Bottom,
}
pub struct PipePlugin;
impl Plugin for PipePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(move_pipes.system())
            .add_system(spawn_pipes.system())
            .insert_resource(PipeSpawnTimer(Timer::from_seconds(1., true)));
    }
}

pub struct Gap {
    top: f32,
    bottom: f32,
}
impl Default for Gap {
    fn default() -> Self {
        let mut rng = thread_rng();
        let top: f32;
        let bottom: f32;
        let rand: f32 = rng.gen_range(-64.0..=64.0);

        if rand >= 0.0 {
            top = rand;
            bottom = rand - 120.
        } else if rand < -0.0 {
            bottom = rand;
            top = rand + 120.
        } else {
            top = 120.;
            bottom = 0.;
        }

        Self {
            top: top,
            bottom: bottom,
        }
    }
}
fn move_pipes(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut query: Query<(&mut Transform, Entity, &Pipe)>,
    mut scoreboard: ResMut<Score>,
) {
    if game_state.0 == GameStates::GameActive {
        for (mut transform, entity, pipe) in query.iter_mut() {
            transform.translation.x -= 2.;
            if transform.translation.x < -240. - 64. {
                commands.entity(entity).despawn();
            }
            if transform.translation.x == 0. {
                match pipe {
                    Pipe::Top => scoreboard.0 += 1,
                    Pipe::Bottom => (),
                };
            }
        }
    } else if game_state.0 == GameStates::PreGame {
        for (_, entity, _) in query.iter_mut() {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_pipes(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_state: Res<GameState>,
    time: Res<Time>,
    mut timer: ResMut<PipeSpawnTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        //spawn_two pipes
        if game_state.0 == GameStates::GameActive {
            let gap: Gap = Gap::default();

            //top
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(Color::rgb(0.10, 0.85, 0.2).into()),
                    sprite: Sprite::new(Vec2::new(64., 640.)),
                    transform: Transform {
                        translation: Vec3::new(240., gap.top + (640. / 2.), 1.).into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Pipe::Top)
                .insert(Gap {
                    top: gap.top,
                    bottom: gap.bottom,
                })
                .insert(Collider);
            //bottom
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(Color::rgb(0.10, 0.85, 0.2).into()),
                    sprite: Sprite::new(Vec2::new(64., 640.)),
                    transform: Transform {
                        translation: Vec3::new(240., gap.bottom - (640. / 2.), 1.).into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Pipe::Bottom)
                .insert(Gap {
                    top: gap.top,
                    bottom: gap.bottom,
                })
                .insert(Collider);
        }
    }
}
