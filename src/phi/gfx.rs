use phi::data::Rectangle;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use sdl2::render::{Renderer, Texture};
use sdl2_image::LoadTexture;


#[derive(Clone)]
pub struct Sprite {
    tex: Rc<RefCell<Texture>>,
    src: Rectangle,
}