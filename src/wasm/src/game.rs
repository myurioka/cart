mod cart;
mod music;
mod ornament;
mod wall;
use crate::{
    browser::now,
    engine::{Audio, Game, KeyState, Line, Point, Renderer, Sound, Velocity},
    game::wall::wall::WALLS_DATA,
};
use anyhow::Result;
use async_trait::async_trait;
use cart::cart::*;
use music::music::*;
use ornament::ornament::*;
use wall::wall::*;
/* <-- CONSTANT VALUE */

pub const CANVAS_WIDTH: f32 = 800.0;
pub const CANVAS_HEIGHT: f32 = 1000.0;
const CART_START_X: f32 = CANVAS_WIDTH / 2.0;
const CART_START_Y: f32 = 100.0;

const FONT_COLOR: &str = "green";
const STAGE_LEFT: f32 = 100.0;
const STAGE_GOAL: f32 = 4200.0;
const VELOCITY_X: f32 = 0.8;
const VELOCITY_STEP: f32 = 0.03;
const VELOCITY_BRAKE_STEP: f32 = 0.06;
const VELOCITY_LIMIT: f32 = 5.0;
const VELOCITY_ZERO: f32 = 0.0;
/// SCREEN
const TITLE: &str = "Cart";
const TITLE_X: f32 = CANVAS_WIDTH / 2.0;
const TITLE_Y: f32 = 820.0;
const TITLE_MESSAGE: &str = "Push Space Key.";
const TITLE_MESSAGE_X: f32 = CANVAS_WIDTH / 2.0;
const TITLE_MESSAGE_Y: f32 = 660.0;

const MESSAGE_TIME_X: f32 = 100.0;
const MESSAGE_TIME_Y: f32 = 880.0;
const MESSAGE_HIGHSCORE_X: f32 = 110.0;
const MESSAGE_HIGHSCORE_Y: f32 = 940.0;
const MESSAGE_VELOCITY_X: f32 = 110.0;
const MESSAGE_VELOCITY_Y: f32 = 840.0;
const MESSAGE_TIME: i32 = 100;
const MESSAGE_RUNNING: &str = "Ready Go!";
const MESSAGE_GAMEOVER: &str = "Game Over!";
const MESSAGE_GAMECLEAR: &str = "Congrantuation!!";
const MESSAGE_DISTANCE: f32 = 120.0;
const ORNAMENT_X: f32 = STAGE_LEFT + 20.0;
const ORNAMENT_Y: f32 = 950.0;
const ORNAMENT_WIDTH: f32 = 10.0;
const ORNAMENT_HEIGHT: f32 = 9900.0;
const BRAKESOUND_FILE: &str = "/cart/assets/beep-7.wav";
const BACKGROUND_MUSIC_FILE: &str = "/cart/assets/background_song.mp3";

/* CONSTANT VALUE --> */

pub struct GameStage {
    machine: Option<GameStageStateMachine>,
}
impl GameStage {
    /// Create a new instance of the game stage
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
    /// Create a new game state machine and initialize with Ready state
    fn new(material: Material) -> Self {
        GameStageStateMachine::Ready(GameStageState::new(material))
    }
    /// Update game state based on key input
    fn update(self, _keystate: &KeyState) -> Self {
        match self {
            GameStageStateMachine::Ready(state) => state.update(_keystate).into(),
            GameStageStateMachine::Playing(state) => state.update(_keystate).into(),
            GameStageStateMachine::GameOver(state) => state.update(_keystate).into(),
            GameStageStateMachine::GameClear(state) => state.update(_keystate).into(),
        }
    }
    /// Draw game elements based on current state
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
    /// Create new game state in Ready state
    fn new(material: Material) -> GameStageState<Ready> {
        GameStageState {
            _state: Ready,
            material,
        }
    }
    /// Start game and transition to Playing state
    fn start_running(self) -> GameStageState<Playing> {
        GameStageState {
            _state: Playing,
            material: self.material,
        }
    }
    /// Handle key input in Ready state (start game with space key)
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
    /// Main update process during gameplay
    fn update(mut self, _keystate: &KeyState) -> RunningEndState {
        let mut _position: Point = self.material.cart.get_position();
        let mut _velocity: Velocity = self.material.cart.get_velocity();

        self.material.distance += _velocity.y;

        // Cart reach goal
        if self.material.distance > STAGE_GOAL {
            let mut _highscore: i32 = now().unwrap() as i32 - self.material.start_time;
            if self.material.highscore != 0 {
                _highscore = _highscore.min(self.material.highscore);
            }
            self.material.highscore = _highscore;
            self.material.score = now().unwrap() as i32 - self.material.start_time;
            return RunningEndState::GameClear(GameStageState {
                _state: GameClear,
                material: self.material,
            });
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
            ornament.run(Velocity {
                x: 0.0,
                y: _velocity.y,
            });
        });

        // walls update
        self.material.walls.iter_mut().for_each(|wall| {
            wall.run(Velocity {
                x: 0.0,
                y: _velocity.y,
            });
        });

        // Check Cart collision
        let _knocked = false;
        for i in 0..self.material.walls.len() {
            let _wall = &self.material.walls[i];
            let _line = _wall.get_line();
            if _wall.p().y.min(_wall.q().y) > self.material.distance {
                continue;
            }
            if self.material.cart.intersect(_line) {
                self.material.cart.knocked();
                return RunningEndState::GameOver(GameStageState {
                    _state: GameOver,
                    material: self.material,
                });
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
    /// Handle GameOver state (restart with space key)
    fn update(self, _keystate: &KeyState) -> GameOverEndState {
        if _keystate.is_pressed("Space") {
            GameOverEndState::Complete(self.new_game())
        } else {
            GameOverEndState::Continue(self)
        }
    }
    /// Start new game (reset materials to Ready state)
    fn new_game(self) -> GameStageState<Ready> {
        GameStageState {
            _state: Ready,
            material: Material::reset(self.material),
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
    /// Handle GameClear state (restart with space key)
    fn update(self, _keystate: &KeyState) -> GameClearEndState {
        if _keystate.is_pressed("Space") {
            GameClearEndState::Complete(self.new_game())
        } else {
            GameClearEndState::Continue(self)
        }
    }
    /// Start new game (reset materials to Ready state)
    fn new_game(self) -> GameStageState<Ready> {
        GameStageState {
            _state: Ready,
            material: Material::reset(self.material),
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
    pub velocity: Velocity,
}
impl Context {
    /// Update context (currently changes nothing)
    fn update(self) -> Self {
        self
    }
    /// Update context with specified velocity
    fn run(mut self, velocity: Velocity) -> Self {
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
    /// Get reference to state context
    pub fn context(&self) -> &Context {
        &self.context
    }
    /// Update state context
    fn update_context(&mut self) {
        self.context = self.context.update();
    }
}
pub enum Event {
    Run(Velocity),
    Update,
}

#[derive(Copy, Clone)]
pub enum StateMachine {
    Running(State<Running>),
}
#[derive(Copy, Clone)]
pub struct Running;
impl State<Running> {
    /// Create new Running state
    pub fn new(p: Point, q: Point, velocity: Velocity) -> Self {
        State {
            context: Context {
                p: p,
                q: q,
                velocity: velocity,
            },
            _state: Running {},
        }
    }

    /// Update position based on velocity
    pub fn update(mut self) -> State<Running> {
        self.context.p = self.context.p.add(self.context.velocity);
        self.context.q = self.context.q.add(self.context.velocity);
        self.update_context();
        self
    }
    /// Run with specified velocity
    pub fn run(self, velocity: Velocity) -> State<Running> {
        State {
            context: self.context.run(velocity),
            _state: Running {},
        }
    }
}

impl StateMachine {
    /// Execute state transition based on event
    fn transition(self, event: Event) -> Self {
        match (self, event) {
            (StateMachine::Running(state), Event::Run(velocity)) => state.run(velocity).into(),
            (StateMachine::Running(state), Event::Update) => state.update().into(),
        }
    }
    /// Get state machine context
    pub fn context(&self) -> &Context {
        match self {
            StateMachine::Running(state) => state.context(),
        }
    }
    /// Update state machine using update event
    fn update(self) -> Self {
        self.transition(Event::Update)
    }
}
impl From<State<Running>> for StateMachine {
    fn from(state: State<Running>) -> Self {
        StateMachine::Running(state)
    }
}

pub trait Piece {
    fn new(p: Point, q: Point, velocity: Velocity) -> Self;
    fn get_state_machine(&self) -> StateMachine;
    fn set_state_machine(&mut self, state_machine: StateMachine);
    fn update(&mut self) {
        let _state_machine = self.get_state_machine();
        self.set_state_machine(_state_machine);
    }
    fn run(&mut self, velocity: Velocity) {
        let _from_state_machine = self.get_state_machine();
        let _to_state_machine = _from_state_machine.transition(Event::Run(velocity));
        self.set_state_machine(_to_state_machine);
    }
    fn get_line(&self) -> Line {
        Line::new(
            Point::new(
                self.get_state_machine().context().p.x,
                self.get_state_machine().context().p.y,
            ),
            Point::new(
                self.get_state_machine().context().q.x,
                self.get_state_machine().context().q.y,
            ),
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
    start_time: i32,
    distance: f32,
    highscore: i32,
    score: i32,
    cart: Cart,
    ornaments: Vec<Ornament>,
    walls: Vec<Wall>,
}
impl Material {
    /// Create new game materials (set highscore, audio, and sound)
    fn new(highscore: i32, audio: Audio, sound: Sound) -> Self {
        let mut _walls = vec![];
        let _start_time: i32 = now().unwrap() as i32;
        for w in WALLS_DATA {
            _walls.push(Wall::new(
                Point { x: w.0, y: w.1 },
                Point { x: w.2, y: w.3 },
                Velocity { x: 0.0, y: 0.0 },
            ));
        }
        Material {
            music: Music::new(audio, sound),
            distance: 0.0,
            start_time: _start_time,
            highscore: highscore,
            score: 0,
            cart: Cart::new(
                Point {
                    x: CART_START_X,
                    y: CART_START_Y,
                },
                Velocity { x: 0.0, y: 0.0 },
            ),
            ornaments: vec![Ornament::new(
                Point {
                    x: ORNAMENT_X,
                    y: ORNAMENT_Y,
                },
                Point {
                    x: ORNAMENT_X + ORNAMENT_WIDTH,
                    y: ORNAMENT_Y + ORNAMENT_HEIGHT,
                },
                Velocity { x: 0.0, y: 0.0 },
            )],
            walls: _walls,
        }
    }
    /// Reset game materials (keep highscore)
    fn reset(material: Self) -> Self {
        Material::new(
            material.highscore,
            material.music.audio.clone(),
            material.music.sound.clone(),
        )
    }
    /// Draw all game elements
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
    /// Initialize game and set up audio and game materials
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
                    _walls.push(Wall::new(
                        Point { x: w.0, y: w.1 },
                        Point { x: w.2, y: w.3 },
                        Velocity { x: 0.0, y: 0.0 },
                    ));
                }
                let machine = GameStageStateMachine::new(Material {
                    start_time: 0,
                    distance: 0.0,
                    highscore: 0,
                    score: 0,
                    music: Music::new(audio, sound),
                    cart: Cart::new(
                        Point {
                            x: CART_START_X,
                            y: CART_START_Y,
                        },
                        Velocity { x: 0.0, y: 0.0 },
                    ),
                    ornaments: vec![Ornament::new(
                        Point {
                            x: ORNAMENT_X,
                            y: ORNAMENT_Y,
                        },
                        Point {
                            x: ORNAMENT_X + ORNAMENT_WIDTH,
                            y: ORNAMENT_Y + ORNAMENT_HEIGHT,
                        },
                        Velocity { x: 0.0, y: 0.0 },
                    )],
                    walls: _walls,
                });
                Ok(Box::new(GameStage {
                    machine: Some(machine),
                }))
            }
        }
    }

    /// Update entire game
    fn update(&mut self, _keystate: &KeyState) {
        if let Some(machine) = self.machine.take() {
            self.machine.replace(machine.update(_keystate));
        }

        //assert!(self.machine.is_some());
    }
    // Draw the entire game
    fn draw(&self, renderer: &Renderer) {
        renderer.clear(&Point { x: 0.0, y: 0.0 }, CANVAS_WIDTH, CANVAS_HEIGHT);
        match &self.machine {
            Some(GameStageStateMachine::Ready(_state)) => {
                draw_openning(renderer);
            }
            Some(GameStageStateMachine::Playing(_state)) => {
                let _time = now().unwrap() as i32 - _state.material.start_time;
                renderer.text(
                    &Point {
                        x: MESSAGE_HIGHSCORE_X,
                        y: MESSAGE_HIGHSCORE_Y,
                    },
                    format!("BEST TIME: {}", get_passed_time(&_state.material.highscore)).as_str(),
                    FONT_COLOR,
                    "32px myfont",
                    "left",
                );
                renderer.text(
                    &Point {
                        x: MESSAGE_TIME_X,
                        y: MESSAGE_TIME_Y,
                    },
                    format!("Time: {}", get_passed_time(&_time)).as_str(),
                    FONT_COLOR,
                    "32px selif",
                    "left",
                );
                renderer.text(
                    &Point {
                        x: MESSAGE_VELOCITY_X,
                        y: MESSAGE_VELOCITY_Y,
                    },
                    format!("Velocity: {:.1}", _state.material.cart.get_velocity().y).as_str(),
                    FONT_COLOR,
                    "32px selif",
                    "left",
                );
                if _time < MESSAGE_TIME {
                    renderer.text(
                        &Point {
                            x: TITLE_MESSAGE_X,
                            y: TITLE_MESSAGE_Y,
                        },
                        MESSAGE_RUNNING,
                        FONT_COLOR,
                        "32px myfont",
                        "center",
                    );
                }
            }
            Some(GameStageStateMachine::GameOver(_state)) => {
                let _score = now().unwrap() as i32 - _state.material.start_time;
                let mut _message = MESSAGE_GAMEOVER.to_string();
                draw_gameover(renderer);
            }
            Some(GameStageStateMachine::GameClear(_state)) => {
                renderer.text(
                    &Point {
                        x: TITLE_MESSAGE_X,
                        y: TITLE_MESSAGE_Y,
                    },
                    MESSAGE_GAMECLEAR,
                    FONT_COLOR,
                    "48px my_font",
                    "center",
                );
                let _message = format!("Your Time: {} s", get_passed_time(&_state.material.score));
                renderer.text(
                    &Point {
                        x: TITLE_MESSAGE_X,
                        y: TITLE_MESSAGE_Y - MESSAGE_DISTANCE,
                    },
                    &_message,
                    FONT_COLOR,
                    "32px my_font",
                    "center",
                );
            }
            _ => {}
        }
        if let Some(machine) = &self.machine {
            machine.draw(renderer);
        }
    }
}

fn get_passed_time(secondtime: &i32) -> String {
    let _seconds = secondtime / 1000;
    let _mini_seconds = secondtime % 1000;
    format!("{:<02}.{:<02}", _seconds, _mini_seconds)
}

fn draw_gameover(renderer: &Renderer) {
    renderer.text(
        &Point {
            x: TITLE_MESSAGE_X,
            y: TITLE_MESSAGE_Y,
        },
        MESSAGE_GAMEOVER,
        FONT_COLOR,
        "48px myfont",
        "center",
    );
    renderer.text(
        &Point {
            x: TITLE_MESSAGE_X,
            y: TITLE_MESSAGE_Y - MESSAGE_DISTANCE - 10.0,
        },
        TITLE_MESSAGE,
        FONT_COLOR,
        "48px myfont",
        "center",
    );
}
fn draw_openning(renderer: &Renderer) {
    renderer.text(
        &Point {
            x: TITLE_X,
            y: TITLE_Y,
        },
        TITLE,
        FONT_COLOR,
        "120px myfont",
        "center",
    );
    renderer.text(
        &Point {
            x: TITLE_MESSAGE_X,
            y: TITLE_MESSAGE_Y,
        },
        TITLE_MESSAGE,
        FONT_COLOR,
        "48px myfont",
        "center",
    );
    renderer.text(
        &Point {
            x: TITLE_MESSAGE_X,
            y: TITLE_MESSAGE_Y - MESSAGE_DISTANCE + 10.0,
        },
        "Speed Up",
        FONT_COLOR,
        "36px selif",
        "center",
    );
    renderer.text(
        &Point {
            x: TITLE_MESSAGE_X,
            y: TITLE_MESSAGE_Y - MESSAGE_DISTANCE - 30.0,
        },
        "▲",
        FONT_COLOR,
        "36px selif",
        "center",
    );
    renderer.text(
        &Point {
            x: TITLE_MESSAGE_X - MESSAGE_DISTANCE,
            y: TITLE_MESSAGE_Y - MESSAGE_DISTANCE - 80.0,
        },
        "To Left ◀",
        FONT_COLOR,
        "36px selif",
        "center",
    );
    renderer.text(
        &Point {
            x: TITLE_MESSAGE_X + MESSAGE_DISTANCE + 5.0,
            y: TITLE_MESSAGE_Y - MESSAGE_DISTANCE - 80.0,
        },
        "▶ To Right",
        FONT_COLOR,
        "36px selif",
        "center",
    );
    renderer.text(
        &Point {
            x: TITLE_MESSAGE_X,
            y: TITLE_MESSAGE_Y - MESSAGE_DISTANCE - 130.0,
        },
        "▼",
        FONT_COLOR,
        "36px selif",
        "center",
    );
    renderer.text(
        &Point {
            x: TITLE_MESSAGE_X,
            y: TITLE_MESSAGE_Y - MESSAGE_DISTANCE - 170.0,
        },
        "Straighten",
        FONT_COLOR,
        "36px selif",
        "center",
    );
    renderer.text(
        &Point {
            x: TITLE_MESSAGE_X,
            y: TITLE_MESSAGE_Y - MESSAGE_DISTANCE - 240.0,
        },
        "[   SPACE   ]",
        FONT_COLOR,
        "24 myfont",
        "center",
    );
    renderer.text(
        &Point {
            x: TITLE_MESSAGE_X,
            y: TITLE_MESSAGE_Y - MESSAGE_DISTANCE - 300.0,
        },
        "Brake",
        FONT_COLOR,
        "24 myfont",
        "center",
    );
}
