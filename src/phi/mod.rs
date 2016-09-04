#[macro_use]
mod events;
pub mod gfx;
pub mod data;

use self::gfx::Sprite;
use sdl2::render::Renderer;
use sdl2::pixels::Color;
use std::collections::HashMap;
use std::path::Path;

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_left: Left,
        key_right: Right,
        key_space: Space
    },
    else: {
        quit: Quit { .. }
    }
}


/// Bundles the Phi abstractions in a single structure which
/// can be passed easily between functions.
pub struct Phi<'window> {
    pub events: Events,
    pub renderer: Renderer<'window>,
    cached_fonts: HashMap<(&'static str, i32), ::sdl2_ttf::Font>,
    ttf_context: ::sdl2_ttf::Sdl2TtfContext,
}

impl<'window> Phi<'window> {
    fn new(events: Events, renderer: Renderer<'window>, ttf_context: ::sdl2_ttf::Sdl2TtfContext) -> Phi<'window> {
        Phi {
            events: events,
            renderer: renderer,
            cached_fonts: HashMap::new(),
            ttf_context: ttf_context,
        }
    }

    pub fn output_size(&self) -> (f64, f64) {
        let (w, h) = self.renderer.output_size().unwrap();
        (w as f64, h as f64)
    }

    pub fn ttf_str_sprite(&mut self, text: &str, font_path: &'static str, size: i32, color: Color) -> Option<Sprite> {
        if let Some(font) = self.cached_fonts.get(&(font_path, size)){
            return font.render(text).blended(color).ok()
            .and_then(|surface| self.renderer.create_texture_from_surface(&surface).ok())
            .map(Sprite::new);
        }
        self.ttf_context.load_font(Path::new(&font_path), size as u16).ok()
        .and_then(|font|{
            self.cached_fonts.insert((font_path, size), font);
            self.ttf_str_sprite(text, font_path, size, color)
        })
    }
}


/// A `ViewAction` is a way for the currently executed view to
/// communicate with the game loop. It specifies which action
/// should be executed before the next rendering.
pub enum ViewAction {
    None,
    Quit,
    ChangeView(Box<View>),
}


pub trait View {
    /// Called on every frame to take care of both the logic and
    /// the rendering of the current view.
    ///
    /// `elapsed` is expressed in seconds.
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}


pub fn spawn<F>(title: &str, init: F)
    where F: Fn(&mut Phi) -> Box<View> {
    // Initialize SDL2
    let sdl_context = ::sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();
    let ttf_context = ::sdl2_ttf::init().unwrap();

    // Create the window
    let window = video.window(title, 800, 600)
        .position_centered().opengl().resizable()
        .build().unwrap();

    // Create the context
    let mut context = Phi::new (
        Events::new(sdl_context.event_pump().unwrap()),
        window.renderer()
            .accelerated()
            .build().unwrap(),
        ttf_context,
    );

    // Create the default view
    let mut current_view = init(&mut context);


    // Frame timing  使用的上一秒帧率

    let interval = 1_000 / 60;//间隔 60帧，每帧多长时间？
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;

    loop {
        // Frame timing (bis)

        let now = timer.ticks();
        let dt = now - before;
        let elapsed = dt as f64 / 1_000.0;//过去的时间

        // If the time elapsed since the last frame is too small, wait out the
        // difference and try again.
        if dt < interval {
            timer.delay(interval - dt);
            continue;
        }

        before = now;
        fps += 1;

        if now - last_second > 1_000 {
            //println!("FPS: {}", fps);
            last_second = now;
            fps = 0;
        }


        // Logic & rendering

        context.events.pump(&mut context.renderer);

        match current_view.render(&mut context, elapsed) {
            ViewAction::None => context.renderer.present(),
            ViewAction::Quit => break,
            ViewAction::ChangeView(new_view) =>
                current_view = new_view,
        }
    }
}