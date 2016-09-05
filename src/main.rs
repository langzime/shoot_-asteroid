extern crate rand;
extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;
extern crate sdl2_mixer;

mod phi;
mod views;


fn main() {
    ::phi::spawn("learn rust", |phi| {
        Box::new(::views::main_menu::MainMenuView::new(phi))
    });
}