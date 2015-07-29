extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event::{Event, Events, EventLoop, RenderEvent};
use piston::input::{Button, Input, Key};
use piston::window::WindowSettings;
use graphics::{Context, Ellipse};

mod color {
    pub const VIOLET: [f32; 4] = [0.6, 0.0, 1.0, 1.0];
    pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
}

struct Game {
    ballX: f64,
    ballY: f64,
    ballSpeedX: f64,
    //ballSpeedY: f64
    width: f64,
    height: f64
}

impl Game {
    fn new(width: f64, height: f64) -> Game {
        Game {
            ballX: width / 2.0,
            ballY: height / 2.0,
            ballSpeedX: 5.0,
            width: width,
            height: height
        }
    }

    fn reset(&mut self) {

    }

    fn render(&mut self, c: Context, gl: &mut GlGraphics) {
        graphics::clear(color::BLACK, gl);

        let radius = 5.0 * 2.0;
        Ellipse::new(color::VIOLET).draw(
            [self.ballX - radius, self.ballY - radius, radius * 2.0, radius * 2.0],
            &c.draw_state, c.transform, gl);
    }

    fn update(&mut self, dt: f64) {
        self.ballX = self.ballX + self.ballSpeedX;
        if self.ballX > self.width {
            self.ballSpeedX = -self.ballSpeedX;
        }
        if self.ballX < 0.0 {
            self.ballSpeedX = -self.ballSpeedX;
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let width = 1024;
    let height = 600;
    let window: GlutinWindow =
        WindowSettings::new("Pong", [width, height])
            .exit_on_esc(true)
            .opengl(opengl)
            .samples(8) // TODO: Check this line
            .vsync(true)
            .into();    // TODO: Check this line

    let mut gl = GlGraphics::new(opengl);

    let mut game = Game::new(width as f64, height as f64);

    for e in window.events().ups(60).max_fps(60) {
        match e {
            Event::Input(Input::Release(Button::Keyboard(key))) => {
                match key {
                    Key::Space => game.reset(),
                    _ => ()
                }
            }

            Event::Render(args) => {
                gl.draw(args.viewport(), |c, g| game.render(c, g));
            }

            Event::Update(args) => {
                game.update(args.dt);
            }

            _ => {}
        }
    }
}

