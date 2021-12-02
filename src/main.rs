use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use rand::prelude::*;

mod pipe;
mod player;

use pipe::PipePlugin;
use player::PlayerPlugin;

const GRAVITY: f32 = 9.8;

struct InputReady(bool);

struct Ground;

struct GameState(GameStates);

impl Default for GameState {
    fn default() -> Self {
        Self(GameStates::PreGame)
    }
}
#[derive(PartialEq, Eq)]
pub enum GameStates {
    PreGame,
    GameActive,
    GameOver,
    Reset,
}

pub struct Collider;

struct Score(i32);
struct ScoreBoard;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Bevy Birds".to_string(),
            width: 480.,
            height: 640.,
            ..Default::default()
        })
        .insert_resource(GameState::default())
        .insert_resource(Score(0))
        .insert_resource(InputReady(true))
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(PipePlugin)
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_system(update_game_state.system())
        .add_system(update_scoreboard.system())
        .add_system(reset_score.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let window = windows.get_primary_mut().unwrap();

    let bottom = window.height() / 2.;

    //ground
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.59, 0.29, 0.0).into()),
            sprite: Sprite::new(Vec2::new(480., 40.)),
            transform: Transform {
                translation: Vec3::new(0., -bottom + 20., 1.).into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Ground)
        .insert(Collider);

    //scoreboard
    commands
        .spawn_bundle(TextBundle {
            text: Text{
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle{
                            font: asset_server.load("fonts/FiraMono-Bold.otf"),
                            font_size: 40.,
                            color: Color::rgb(1.,1.,1.)
                        }
                    },
                    TextSection {
                        value:"".to_string(),
                        style: TextStyle{
                            font: asset_server.load("fonts/FiraMono-Medium.otf"),
                            font_size: 40.,
                            color: Color::rgb(1.,1.,1.)
                        }
                    }
                ],
                ..Default::default()  
            },
            style: Style{
                position_type: PositionType::Absolute,
                position: Rect{
                    top: Val::Px(5.),
                    bottom: Val::Px(5.),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }).insert(ScoreBoard);
}

fn update_scoreboard(score: Res<Score>, mut query: Query<(&mut Text, With<ScoreBoard>)>){
    if let Ok((mut text, _)) = query.single_mut(){
        text.sections[1].value = format!("{}", score.0);
    }
}
fn reset_score(mut score: ResMut<Score>, state: Res<GameState>){
    if state.0 == GameStates::PreGame && score.0 > 0{
        score.0 = 0;
    }
}
fn update_game_state(
    mut game_state: ResMut<GameState>,
    input: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut ready: ResMut<InputReady>,
) {
    if ready.0 && (input.pressed(KeyCode::Space) || mouse.pressed(MouseButton::Left) ) {
        match game_state.0 {
            GameStates::PreGame => game_state.0 = GameStates::GameActive,
            GameStates::GameOver => game_state.0 = GameStates::Reset,
            _ => (),
        }
        ready.0 = false;
    }
    if input.just_released(KeyCode::Space) || mouse.just_released(MouseButton::Left) {
        ready.0 = true;
    }
}
