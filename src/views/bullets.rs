use phi::data::Rectangle;
use phi::Phi;
use sdl2::pixels::Color;

/// Pixels traveled horizontally by a bullet every second.
const BULLET_SPEED: f64 = 240.0;
const BULLET_W: f64 = 8.0;
const BULLET_H: f64 = 4.0;


pub trait Bullet {
    /// Update the bullet.
    /// If the bullet should be destroyed, e.g. because it has left the screen,
    /// then return `None`.
    /// Otherwise, return `Some(update_bullet)`.
    fn update(self: Box<Self>, phi: &mut Phi, dt: f64) -> Option<Box<Bullet>>;

    /// Render the bullet to the screen.
    fn render(&self, phi: &mut Phi);

    /// Return the bullet's bounding box.
    fn rect(&self) -> Rectangle;
}

#[derive(Clone, Copy)]
struct RectBullet {
    rect: Rectangle,
}

impl Bullet for RectBullet {
    /// Update the bullet.
    /// If the bullet should be destroyed, e.g. because it has left the screen,
    /// then return `None`.
    /// Otherwise, return `Some(update_bullet)`.
    fn update(mut self: Box<Self>, phi: &mut Phi, dt: f64) -> Option<Box<Bullet>> {
        let (w, _) = phi.output_size();
        self.rect.x += BULLET_SPEED * dt;

        // If the bullet has left the screen, then delete it.
        if self.rect.x > w {
            None
        } else {
            Some(self)
        }
    }

    /// Render the bullet to the screen.
    fn render(&self, phi: &mut Phi) {
        // We will render this kind of bullet in yellow.
        //? This is exactly how we drew our first moving rectangle in the
        //? seventh part of this series.
        phi.renderer.set_draw_color(Color::RGB(230, 230, 30));
        phi.renderer.fill_rect(self.rect.to_sdl().unwrap());
    }

    /// Return the bullet's bounding box.
    fn rect(&self) -> Rectangle {
        self.rect
    }
}

pub fn spawn_bullets( cannons_x: f64,
                      cannon1_y: f64,
                      cannon2_y: f64) -> Vec<Box<Bullet>> {
    let cannons_x = cannons_x;
    let cannon1_y = cannon1_y;
    let cannon2_y = cannon2_y;

    // One bullet at the tip of every cannon
    //? We could modify the initial position of the bullets by matching on
    //? `self.current : ShipFrame`, however there is not much point to this
    //? pedagogy-wise. You can try it out if you want. ;)
    vec![
            Box::new(RectBullet {
                rect: Rectangle {
                    x: cannons_x,
                    y: cannon1_y,
                    w: BULLET_W,
                    h: BULLET_H,
                }
            }),
            Box::new(RectBullet {
                rect: Rectangle {
                    x: cannons_x,
                    y: cannon2_y,
                    w: BULLET_W,
                    h: BULLET_H,
                }
            })
        ]
}