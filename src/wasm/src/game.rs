mod cart;
mod ornament;
mod wall;
mod music;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use cart::cart::*;
use ornament::ornament::*;
use wall::wall::*;
use music::music::*;
use crate::{
    game::wall::wall::WALLS_DATA,
    engine::{Audio, Sound, Line, Point, Game, KeyState, Renderer}
};
/* <-- CONSTANT VALUE */
const STAGE_LEFT: f32 = 100.0;
const STAGE_GOAL:f32 = 2300.0;
const CART_START_X:f32 = 220.0;
const CART_START_Y:f32 = 70.0;
const VELOCITY_X:f32 = 0.8;
const VELOCITY_STEP:f32 = 0.2;
const VELOCITY_BRAKE_STEP:f32 = 0.1;
const VELOCITY_LIMIT: f32 = 5.0;
const VELOCITY_ZERO: f32 = 0.0;
const ORNAMENT_X: f32 = STAGE_LEFT + 20.0;
const ORNAMENT_Y: f32 = 50.0;
const ORNAMENT_WIDTH: f32 = 10.0;
const ORNAMENT_HEIGHT: f32 = 10.0;
const MESSAGE_HIGHSCORE_X: f32 = 40.0;
const MESSAGE_HIGHSCORE_Y: f32 = 570.0;
const MESSAGE_TIME_X: f32 = 40.0;
const MESSAGE_TIME_Y: f32 = 540.0;
const MESSAGE_VELOCITY_X: f32 = 40.0;
const MESSAGE_VELOCITY_Y: f32 = 510.0;
const MESSAGE_TIME: f32 =100.0;
const MESSAGE_X: f32 = 230.0;
const MESSAGE_Y: f32 = 350.0;
const MESSAGE_OPENING_Y: f32 = 130.0;
const MESSAGE_OPENING: &str = "Push Space Key.";
const MESSAGE_RUNNING: &str = "Start!!";
const MESSAGE_GAMEOVER: &str = "Game OVER!!";
const MESSAGE_GAMECLEAR: &str = "Congrantuation!!";
const MESSAGE_DISTANCE: f32 = 30.0;
const BRAKESOUND_FILE: &str = "/cart/assets/beep-7.wav";
const BACKGROUND_MUSIC_FILE: &str = "/cart/assets/background_song.mp3";
/* CONSTANT VALUE --> */

pub struct GameStage {
    machine: Option<GameStageStateMachine>,
}
impl GameStage {
    pub fn new() -> Self {
        GameStage { machine: None }
    }
}
enum GameStageStateMachine {
    Ready(GameStageState<Ready>),
    Playing(GameStageState<Playing>),
    GameOver(GameStageState<GameOver>),
    GameClear(GameStageState<GameClear>),
}
impl GameStageStateMachine {
    fn new(material: Material) -> Self {
        GameStageStateMachine::Ready(GameStageState::new(material))
    }
    fn update(self, _keystate: &KeyState) -> Self {
        match self {
            GameStageStateMachine::Ready(state) => state.update(_keystate).into(),
            GameStageStateMachine::Playing(state) => state.update(_keystate).into(),
            GameStageStateMachine::GameOver(state) => state.update(_keystate).into(),
            GameStageStateMachine::GameClear(state) => state.update(_keystate).into(),
        }
    }
    fn draw(&self, renderer: &Renderer) {
        match self {
            GameStageStateMachine::Ready(state) => state.material.draw(renderer),
            GameStageStateMachine::Playing(state) => state.material.draw(renderer),
            GameStageStateMachine::GameOver(state) => state.material.draw(renderer),
            GameStageStateMachine::GameClear(state) => state.material.draw(renderer),
        };
    }
}
impl From<GameStageState<Ready>> for GameStageStateMachine {
    fn from(state: GameStageState<Ready>) -> Self {
        GameStageStateMachine::Ready(state)
    }
}
impl From<GameStageState<Playing>> for GameStageStateMachine {
    fn from(state: GameStageState<Playing>) -> Self {
        GameStageStateMachine::Playing(state)
    }
}
impl From<GameStageState<GameOver>> for GameStageStateMachine {
    fn from(state: GameStageState<GameOver>) -> Self {
        GameStageStateMachine::GameOver(state)
    }
}
impl From<GameStageState<GameClear>> for GameStageStateMachine {
    fn from(state: GameStageState<GameClear>) -> Self {
        GameStageStateMachine::GameClear(state)
    }
}

struct GameStageState<T> {
    _state: T,
    material: Material,
}

struct Ready;
impl GameStageState<Ready> {
    fn new(material: Material) -> GameStageState<Ready> {
        GameStageState { _state: Ready, material,}
    }
    fn start_running(self) -> GameStageState<Playing> {
        GameStageState { _state: Playing, material: self.material,}
    }
    fn update(self, _keystate: &KeyState) -> ReadyEndState {
        if _keystate.is_pressed("Space") {
            return ReadyEndState::Complete(self.start_running());
        }
        ReadyEndState::Continue(self)
    }
}
enum ReadyEndState {
    Complete(GameStageState<Playing>),
    Continue(GameStageState<Ready>),
}
impl From<ReadyEndState> for GameStageStateMachine {
    fn from(state: ReadyEndState) -> Self {
        match state {
            ReadyEndState::Complete(running) => running.into(),
            ReadyEndState::Continue(ready) => ready.into(),
        }
    }
}

struct Playing;
impl GameStageState<Playing> {
    fn update(mut self, _keystate: &KeyState) -> RunningEndState {

        let mut _velocity:Point = self.material.cart.get_velocity();
        let mut _position:Point = self.material.cart.get_position();

        self.material.frame += 1.0;
        self.material.distance += _velocity.y;

        // Cart reach goal
        if self.material.distance > STAGE_GOAL {
            let mut _highscore:f32 = self.material.frame;
            if self.material.highscore != 0.0 {
                _highscore = _highscore.min(self.material.highscore);
            }
            self.material.highscore = _highscore;
            return RunningEndState::GameClear(
                GameStageState {
                    _state: GameClear,
                    material: self.material,
                }
            );
        }
        if _keystate.is_pressed("ArrowUp") && _velocity.y < VELOCITY_LIMIT {
            _velocity.y += VELOCITY_STEP;
        }
        if _keystate.is_pressed("ArrowDown") {
            _velocity.x = 0.0;
        }
        if _keystate.is_pressed("ArrowLeft") {
            _velocity.x = -VELOCITY_X;
        }
        if _keystate.is_pressed("ArrowRight") {
            _velocity.x = VELOCITY_X;
        }
        if _keystate.is_pressed("Space") {
            _velocity.y -= VELOCITY_BRAKE_STEP;
            self.material.music.clone().play_brake_sound();
        }

        // velocity limit
        if _velocity.y < VELOCITY_ZERO {
            _velocity.y = 0.0
        }
        self.material.cart.run(_velocity);

        // Ornament
        self.material.ornaments.iter_mut().for_each(|ornament| {
            ornament.run(Point{x: 0.0, y: -_velocity.y});
        });
        self.material.walls.iter_mut().for_each(|wall| {
            wall.run(Point{x: 0.0, y: -_velocity.y});
        });

        // check Cart collision
        let _knocked = false;
        for i in 0..self.material.walls.len() {
            let _wall = &self.material.walls[i];
            let _line = _wall.get_line();
            if _wall.p().y.min(_wall.q().y) > self.material.distance {
                continue;
            }
            if self.material.cart.intersect(_line) {
                self.material.cart.knocked();
                return RunningEndState::GameOver(
                    GameStageState {
                        _state: GameOver,
                        material: self.material,
                    }
                );
            };
        }
      
        self.material.cart.update();
        self.material.ornaments.iter_mut().for_each(|ornament| {
            ornament.update();
        });
        self.material.walls.iter_mut().for_each(|wall| {
            wall.update();
        });

        RunningEndState::Continue(self)
    }
}
impl From<RunningEndState> for GameStageStateMachine {
    fn from(state: RunningEndState) -> Self {
        match state {
            RunningEndState::Continue(running) => running.into(),
            RunningEndState::GameOver(gameover) => gameover.into(),
            RunningEndState::GameClear(gameclear) => gameclear.into(),
        }
    }
}

struct GameOver;
impl GameStageState<GameOver> {
    fn update(self, _keystate: &KeyState) -> GameOverEndState {
        if _keystate.is_pressed("Space") {
            GameOverEndState::Complete(self.new_game())
        } else {
            GameOverEndState::Continue(self)
        }
    }
    fn new_game(self) -> GameStageState<Ready> {
        GameStageState {
            _state: Ready,
            material: Material::reset(self.material)
        }
    }
}
enum RunningEndState {
    Continue(GameStageState<Playing>),
    GameOver(GameStageState<GameOver>),
    GameClear(GameStageState<GameClear>),
}

enum GameOverEndState {
    Continue(GameStageState<GameOver>),
    Complete(GameStageState<Ready>),
}
impl From<GameOverEndState> for GameStageStateMachine {
    fn from(state: GameOverEndState) -> Self {
        match state {
            GameOverEndState::Continue(game_over) => game_over.into(),
            GameOverEndState::Complete(ready) => ready.into(),
        }
    }
}
struct GameClear;
impl GameStageState<GameClear> {
    fn update(self, _keystate: &KeyState) -> GameClearEndState {
        if _keystate.is_pressed("Space") {
            GameClearEndState::Complete(self.new_game())
        } else {
            GameClearEndState::Continue(self)
        }
    }
    fn new_game(self) -> GameStageState<Ready> {
        GameStageState {
            _state: Ready,
            material: Material::reset(self.material)
        }
    }
}
enum GameClearEndState {
    Continue(GameStageState<GameClear>),
    Complete(GameStageState<Ready>),
}
impl From<GameClearEndState> for GameStageStateMachine {
    fn from(state: GameClearEndState) -> Self {
        match state {
            GameClearEndState::Continue(game_clear) => game_clear.into(),
            GameClearEndState::Complete(ready) => ready.into(),
        }
    }
}


#[derive(Copy, Clone)]
pub struct Context {
    pub p: Point,
    pub q: Point,
    pub velocity: Point,
}
impl Context {
    fn update(self) -> Self {
        self
    }
    fn run(mut self, velocity: Point) -> Self {
        self.velocity = velocity;
        self
    }
}

#[derive(Copy, Clone)]
pub struct State<S> {
    pub context: Context,
    _state: S,
}
impl<S> State<S> {
    pub fn context(&self) -> &Context {
        &self.context
    }
    fn update_context(&mut self){
        self.context = self.context.update();
    }
}
pub enum Event {
    Run(Point),
    Update,
}

#[derive(Copy, Clone)]
pub enum StateMachine{
    Running(State<Running>),
}
#[derive(Copy, Clone)]
pub struct Running;
impl State<Running> {
    pub fn new(p: Point, q: Point, velocity: Point) -> Self {
        State {
            context: Context{
                p: p,
                q: q,
                velocity: velocity,
            },
            _state: Running {},
        }
    }

    pub fn update(mut self)  -> State<Running> {
            self.context.p = self.context.p.add( self.context.velocity);
            self.context.q = self.context.q.add( self.context.velocity);
            self.update_context();
            self
        }
    pub fn run(self, velocity:Point) -> State<Running> {
        State {
            context: self.context.run(velocity),
            _state: Running{},
        }
    }
}

impl StateMachine {
    fn transition(self, event: Event) -> Self {
        match (self, event) {
            (StateMachine::Running(state), Event::Run(velocity)) => state.run(velocity).into(),
            (StateMachine::Running(state), Event::Update) => state.update().into(),
        }
    }
    pub fn context(&self) -> &Context {
        match self {
            StateMachine::Running(state) => state.context(),
        }
    }
    fn update(self) -> Self {
        self.transition(Event::Update)
    }
}
impl From<State<Running>> for StateMachine{
    fn from(state: State<Running>) -> Self {
        StateMachine::Running(state)
    }
}

pub trait Piece {
    fn new(p: Point, q: Point, velocity: Point) -> Self;
    fn get_state_machine(&self) -> StateMachine;
    fn set_state_machine(&mut self, state_machine:StateMachine);
    fn update(&mut self){
        let _state_machine = self.get_state_machine();
        self.set_state_machine(_state_machine);
    }
    fn run(&mut self, velocity:Point){
        let _from_state_machine = self.get_state_machine();
        let _to_state_machine = _from_state_machine.transition(Event::Run(velocity));
        self.set_state_machine(_to_state_machine);
    }
    fn get_line(&self) -> Line {
        Line::new(
            Point::new(self.get_state_machine().context().p.x,
                       self.get_state_machine().context().p.y),
            Point::new(self.get_state_machine().context().q.x,
                        self.get_state_machine().context().q.y)
        )
    }
    fn draw(&self, renderer: &Renderer);
    fn p(&self) -> Point {
        self.get_state_machine().context().p
    }
    fn q(&self) -> Point {
        self.get_state_machine().context().q
    }
}

pub struct Material {
    music: Music,
    frame: f32,
    distance: f32,
    highscore: f32,
    cart: Cart,
    ornaments: Vec<Ornament>,
    walls: Vec<Wall>,
}
impl Material {
    fn new(highscore: f32, audio: Audio, sound: Sound) -> Self {
        let mut _walls = vec![];
        for w in WALLS_DATA {
                _walls.push(Wall::new(Point{x:w.0, y:w.1}, Point{x: w.2, y: w.3},Point{x: 0.0, y: 0.0}));
        };
        Material {
            music: Music::new(
                audio,
                sound,
            ),
            frame: 0.0,
            distance: 0.0,
            highscore: highscore,
            cart: Cart::new(
                Point { x: CART_START_X, y: CART_START_Y },
                Point { x: 0.0, y: 0.0},
            ),
            ornaments: vec![
                Ornament::new(
                    Point { x: ORNAMENT_X, y: ORNAMENT_Y },
                    Point { x: ORNAMENT_X + ORNAMENT_WIDTH, y: ORNAMENT_Y + ORNAMENT_HEIGHT},
                    Point { x: 0.0, y: 0.0 },
            )],
            walls: _walls,
        }
    }
    fn reset(material: Self) -> Self {
        Material::new(
            material.highscore,
            material.music.audio.clone(),
            material.music.sound.clone(),
        )
    }
    fn draw(&self, renderer: &Renderer) {
        self.cart.draw(renderer);
        self.ornaments.iter().for_each(|ornament| {
            ornament.draw(renderer);
        });
        self.walls.iter().for_each(|wall| {
            wall.draw(renderer);
        });
    }
}

#[async_trait(?Send)]
impl Game for GameStage {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        log!("START");
        match &self.machine {
            _none => {
                let audio = Audio::new()?;
                let sound = audio.load_sound(BRAKESOUND_FILE).await?;
                let background_music = audio.load_sound(BACKGROUND_MUSIC_FILE).await?;
                audio.play_looping_sound(&background_music)?;

                let mut _walls = vec![];
                for w in WALLS_DATA {
                    _walls.push(Wall::new(Point{x:w.0, y:w.1}, Point{x: w.2, y: w.3},Point{x: 0.0, y: 0.0}));
                }
                let machine = GameStageStateMachine::new(Material {
                    frame: 0.0,
                    distance: 0.0,
                    highscore: 0.0,
                    music: Music::new(
                        audio,
                        sound,
                    ),
                    cart: Cart::new(
                        Point { x: CART_START_X, y: CART_START_Y },
                        Point { x: 0.0, y: 0.0},
                    ),
                    ornaments: vec![
                        Ornament::new(
                            Point { x: ORNAMENT_X, y: ORNAMENT_Y },
                            Point { x: ORNAMENT_X + ORNAMENT_WIDTH, y: ORNAMENT_Y + ORNAMENT_HEIGHT},
                            Point { x: 0.0, y: 0.0 },
                    )],
                    walls: _walls,
                });
                Ok(Box::new(GameStage {
                    machine: Some(machine),
                }))
            }
            Some(_) => Err(anyhow!("Error: Game is already initialized!")),
            
        }
    }

    // Whole World UPDATE
    fn update(&mut self, _keystate: &KeyState) {
        if let Some(machine) = self.machine.take() {
            self.machine.replace(machine.update(_keystate));
        }
        assert!(self.machine.is_some());
    }
    // Whole World Drawing
    fn draw(&self, renderer: &Renderer) {
        renderer.clear(
            &Point{ x:0.0, y:0.0 },
            600.0,
            600.0,
        );
        match &self.machine {
            Some(GameStageStateMachine::Ready(_state)) => {
                if _state.material.frame < MESSAGE_TIME {
                    renderer.text(
                        &Point {
                            x: MESSAGE_X,
                            y: MESSAGE_OPENING_Y + MESSAGE_DISTANCE * 7.0,
                        },
                        MESSAGE_OPENING,
                    );
                    renderer.text_s(
                        &Point {
                            x: MESSAGE_X ,
                            y: MESSAGE_OPENING_Y + MESSAGE_DISTANCE * 5.0,
                        },
                        "[ ↑ ] SpeedUP",
                    );
                    renderer.text_s(
                        &Point {
                            x: MESSAGE_X,
                            y: MESSAGE_OPENING_Y + MESSAGE_DISTANCE * 4.0,
                        },
                        "[ → ] ToRight",
                    );
                    renderer.text_s(
                        &Point {
                            x: MESSAGE_X,
                            y: MESSAGE_OPENING_Y + MESSAGE_DISTANCE * 3.0,
                        },
                        "[ ← ] ToLeft",
                    );
                    renderer.text_s(
                        &Point {
                            x: MESSAGE_X,
                            y: MESSAGE_OPENING_Y + MESSAGE_DISTANCE * 2.0,
                        },
                        "[ ↓ ] Strait ",
                    );
                    renderer.text_s(
                        &Point {
                            x: MESSAGE_X,
                            y: MESSAGE_OPENING_Y + MESSAGE_DISTANCE,
                        },
                        "[SPACE] Brake ",
                    );
                }
            }
            Some(GameStageStateMachine::Playing(_state)) => {
                renderer.text_left(
                    &Point {
                        x: MESSAGE_HIGHSCORE_X,
                        y: MESSAGE_HIGHSCORE_Y,
                    },
                    format!("HIGHT SCORE: {}", _state.material.highscore).as_str(),
                );
                renderer.text_left(
                    &Point {
                        x: MESSAGE_TIME_X,
                        y: MESSAGE_TIME_Y,
                    },
                    format!("TIME: {}", _state.material.frame).as_str(),
                );
                renderer.text_left(
                    &Point {
                        x: MESSAGE_VELOCITY_X,
                        y: MESSAGE_VELOCITY_Y,
                    },
                    format!("VELOCITY: {:.1}", _state.material.cart.get_velocity().y).as_str(),
                );
                if _state.material.frame < MESSAGE_TIME {
                    renderer.text(
                        &Point {
                            x: MESSAGE_X,
                            y: MESSAGE_Y,
                        },
                        MESSAGE_RUNNING,
                    );
                }
            }
            Some(GameStageStateMachine::GameOver(_state)) => {
                let _score = _state.material.frame;
                let mut _message = MESSAGE_GAMEOVER.to_string();
                renderer.text(
                    &Point {
                        x: MESSAGE_X,
                        y: MESSAGE_Y,
                    },
                    &_message,
                );
            }
            Some(GameStageStateMachine::GameClear(_state)) => {
                let _score = _state.material.frame;
                let mut _message = format!("{} Your Time: {}", MESSAGE_GAMECLEAR, _score);
                renderer.text(
                    &Point {
                        x: MESSAGE_X,
                        y: MESSAGE_Y,
                    },
                    &_message,
                );
            }
            _=> {
            }
        }
        if let Some(machine) = &self.machine {
            machine.draw(renderer);
        }
    }
}