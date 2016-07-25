extern crate sdl2;
extern crate sdl2_image;

mod phi;
mod views;


fn main() {
    ::phi::spawn("learn rust", |phi| {
        Box::new(::views::ShipView::new(phi))
    });
}