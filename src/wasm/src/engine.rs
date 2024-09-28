use crate::browser::{self, LoopClosure};
use crate::sound;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::channel::{
    mpsc::{unbounded, UnboundedReceiver},
    //oneshot::channel,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{AudioContext, AudioBuffer};
use web_sys::CanvasRenderingContext2d;

pub const HEIGHT: f64 = 600.0;
pub const FONT_COLOR: &str = "green";
pub const FONT_M: &str = "18px monospace";
pub const FONT_S: &str = "14px monospace";
pub const FONT_CENTER: &str = "center";
pub const FONT_LEFT: &str = "left";

#[derive(Clone, Copy, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point{
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
    pub fn add(&self, target: Point) -> Point {
        return Point{ x:&self.x + target.x, y:&self.y + target.y};
    }
}

#[derive(Clone, Copy)]
pub struct Line {
    pub p: Point,
    pub q: Point,
}
impl Line {
    pub fn new(p: Point, q: Point) -> Line {
        Line { p: p, q: q }
    }
}

pub struct Renderer {
    context: CanvasRenderingContext2d,
}

impl Renderer {
    pub fn clear(&self, point: &Point, width: f64, height: f64) {
        self.context.clear_rect(
            point.x.into(),
            point.y.into(),
            width,
            height,
        );
    }
    pub fn text(&self, point: &Point, txt: &str) {
        self.context.set_fill_style(&JsValue::from(FONT_COLOR));
        self.context.set_text_align(FONT_CENTER);
        self.context.set_font(FONT_M);
        let _ = self.context.fill_text(txt, point.x, HEIGHT - point.y);
    }
    pub fn text_s(&self, point: &Point, txt: &str) {
        self.context.set_fill_style(&JsValue::from(FONT_COLOR));
        self.context.set_text_align(FONT_CENTER);
        self.context.set_font(FONT_S);
        let _ = self.context.fill_text(txt, point.x, HEIGHT - point.y);
    }
    pub fn text_left(&self, point: &Point, txt: &str) {
        self.context.set_fill_style(&JsValue::from(FONT_COLOR));
        self.context.set_text_align(FONT_LEFT);
        self.context.set_font(FONT_S);
        let _ = self.context.fill_text(txt, point.x, HEIGHT - point.y);
    }
    pub fn line(&self, p: &Point, q: &Point) {
        self.context.begin_path();
        self.context.set_stroke_style(&JsValue::from(FONT_COLOR));
        self.context.move_to(p.x.into(), HEIGHT - p.y);
        self.context.line_to(q.x.into(), HEIGHT - q.y);
        self.context.close_path();
        self.context.stroke();
    }
    pub fn rect(&self, p: &Point, width: f64, height: f64) {
        self.context.set_stroke_style(&JsValue::from(FONT_COLOR));
        self.context.rect(p.x,HEIGHT - p.y, width, height);
        self.context.stroke();
    }
}

#[async_trait(?Send)]
pub trait Game {
    async fn initialize(&self) -> Result<Box<dyn Game>>;
    fn update(&mut self, keystate: &KeyState);
    fn draw(&self, renderer: &Renderer);
}

const FRAME_SIZE: f64 = 1.0 / 30.0 * 1000.0;
pub struct GameLoop {
    last_frame: f64,
    accumulated_delta: f64,
}
type SharedLoopClosure = Rc<RefCell<Option<LoopClosure>>>;

impl GameLoop {
    pub async fn start(game: impl Game + 'static) -> Result<()> {
        let mut keyevent_receiver = prepare_input()?;
        let mut game = game.initialize().await?;
        let mut game_loop = GameLoop {
            last_frame: browser::now()?.into(),
            accumulated_delta: 0.0,
        };

        let renderer = Renderer {
            context: browser::context()?,
        };

        let f: SharedLoopClosure = Rc::new(RefCell::new(None));
        let g = f.clone();

        let mut keystate = KeyState::new();
        *g.borrow_mut() = Some(browser::create_raf_closure(move |perf: f64| {
            process_input(&mut keystate, &mut keyevent_receiver);

            game_loop.accumulated_delta += perf - game_loop.last_frame;
            while game_loop.accumulated_delta > FRAME_SIZE {
                game.update(&keystate);
                game_loop.accumulated_delta -= FRAME_SIZE;
            }
            game_loop.last_frame = perf;
            game.draw(&renderer);

            let _= browser::request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        browser::request_animation_frame(
            g.borrow()
                .as_ref()
                .ok_or_else(|| anyhow!("GameLoop: Loop is None"))?,
        )?;
        Ok(())
    }
}

pub struct KeyState {
    pressed_keys: HashMap<String, web_sys::KeyboardEvent>,
}

impl KeyState {
    fn new() -> Self {
        return KeyState {
            pressed_keys: HashMap::new(),
        };
    }
    pub fn is_pressed(&self, code: &str) -> bool {
        self.pressed_keys.contains_key(code)
    }

    fn set_pressed(&mut self, code: &str, event: web_sys::KeyboardEvent) {
        self.pressed_keys.insert(code.into(), event);
    }

    fn set_released(&mut self, code: &str) {
        self.pressed_keys.remove(code.into());
    }
}

enum KeyPress {
    KeyUp(web_sys::KeyboardEvent),
    KeyDown(web_sys::KeyboardEvent),
}

fn process_input(state: &mut KeyState, keyevent_receiver: &mut UnboundedReceiver<KeyPress>) {
    loop {
        match keyevent_receiver.try_next() {
            Ok(None) => break,
            Err(_err) => break,
            Ok(Some(evt)) => match evt {
                KeyPress::KeyUp(evt) => state.set_released(&evt.code()),
                KeyPress::KeyDown(evt) => state.set_pressed(&evt.code(), evt),
            },
        };
    }
}

// For Keypress Input
fn prepare_input() -> Result<UnboundedReceiver<KeyPress>> {
    let (keydown_sender, keyevent_receiver) = unbounded();
    let keydown_sender = Rc::new(RefCell::new(keydown_sender));
    let keyup_sender = Rc::clone(&keydown_sender);
    let onkeydown = browser::closure_wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        let _ = keydown_sender
            .borrow_mut()
            .start_send(KeyPress::KeyDown(keycode));
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    let onkeyup = browser::closure_wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        let _ = keyup_sender
            .borrow_mut()
            .start_send(KeyPress::KeyUp(keycode));
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    browser::canvas()?.set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));
    browser::canvas()?.set_onkeyup(Some(onkeyup.as_ref().unchecked_ref()));
    onkeydown.forget();
    onkeyup.forget();

    Ok(keyevent_receiver)
}

/* For Mouse Input
pub fn add_click_handler(elem: HtmlElement) -> UnboundedReceiver<()> {
    let (mut click_sender, click_receiver) = unbounded();

    let on_click = browser::closure_wrap(Box::new(move || {
        let _ = click_sender.start_send(());
    }) as Box<dyn FnMut()>);

    elem.set_onclick(Some(on_click.as_ref().unchecked_ref()));
    on_click.forget();
    click_receiver
}
*/

#[derive(Clone)]
pub struct Audio {
    context: AudioContext,
}

#[derive(Clone)]
pub struct Sound {
    pub buffer: AudioBuffer,
}

impl Audio {
    pub fn new() -> Result<Self> {
        Ok(Audio {
            context: sound::create_audio_context()?,
        })
    }

    pub async fn load_sound(&self, filename: &str) -> Result<Sound> {
        let array_buffer = browser::fetch_array_buffer(filename).await?;

        let audio_buffer = sound::decode_audio_data(&self.context, &array_buffer).await?;

        Ok(Sound {
            buffer: audio_buffer,
        })
    }

    pub fn play_sound(&self, sound: &Sound) -> Result<()> {
        sound::play_sound(&self.context, &sound.buffer, sound::LOOPING::No)
    }

    pub fn play_looping_sound(&self, sound: &Sound) -> Result<()> {
        sound::play_sound(&self.context, &sound.buffer, sound::LOOPING::Yes)
    }
}
