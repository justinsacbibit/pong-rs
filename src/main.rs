extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::path::Path;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use opengl_graphics::glyph_cache::GlyphCache;
use piston::input::*;
use piston::event_loop::*;
use piston::window::WindowSettings;
use graphics::{Context, Ellipse, Rectangle, Transformed};

mod color {
    pub const VIOLET: [f32; 4] = [0.6, 0.0, 1.0, 1.0];
    pub const BLACK:  [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    pub const WHITE:  [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    pub const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
}

struct Vector {
    x: f64,
    y: f64
}

struct Ball {
    center: Vector,
    speed: Vector,
    radius: f64
}

impl Ball {
    fn new(center: Vector) -> Ball {
        Ball {
            center: center,
            speed: Vector {
                x: 500.0,
                y: 200.0
            },
            radius: 10.0
        }
    }

    fn update(&mut self, dt: f64) {
        // move the ball according to its current speed and the dt parameter
        self.center.x += self.speed.x * dt;
        self.center.y += self.speed.y * dt;
    }

    fn reset(&mut self, window_width: f64, window_height: f64) {
        // recenter the ball
        self.center.x = window_width / 2.0;
        self.center.y = window_height / 2.0;

        // change up the speed
        self.speed.x = -self.speed.x;
        self.speed.y = -self.speed.y;
    }
}

struct Game {
    ball: Ball,

    window_width: f64,
    window_height: f64,

    paddle_width: f64,
    paddle_height: f64,

    // y-coordinate of center of player 1 (left) paddle
    paddle_1_y: f64,
    // y-coordinate of mouse
    mouse_y: f64,

    paddle_2_y: f64,

    // player 1's score
    score_1: u8,
    score_2: u8,
    showing_win_screen: bool,

    // the score needed to win a game
    winning_score: u8,

    font: GlyphCache<'static>
}

impl Game {
    // constructs a new Game object
    fn new(width: f64, height: f64) -> Game {
        let ball = Ball::new(
            // ball starts in the center of the window
            Vector {
                x: width / 2.0,
                y: height / 2.0
            }
        );

        Game {
            ball: ball,

            window_width: width,
            window_height: height,

            paddle_width: 10.0,
            paddle_height: 100.0,

            // start the paddles vertically centered
            paddle_1_y: height / 2.0,
            mouse_y: height / 2.0,

            paddle_2_y: height / 2.0,

            score_1: 0,
            score_2: 0,
            showing_win_screen: false,

            winning_score: 5,

            font: GlyphCache::new(&Path::new("resources/FiraMono-Bold.ttf")).unwrap()
        }
    }

    // called after a game has been finished and the user has clicked for a new game
    fn reset(&mut self) {
        self.score_1 = 0;
        self.score_2 = 0;
        self.showing_win_screen = false;

        self.ball.reset(self.window_width, self.window_height);
    }

    // called when a player has scored
    fn scored(&mut self, player: u8) {
        // increment the appropriate score
        if player == 2 {
            self.score_2 += 1;
        } else {
            self.score_1 += 1;
        }

        // check if the game is done
        if self.score_1 == self.winning_score || self.score_2 == self.winning_score {
            self.showing_win_screen = true;
        }

        self.ball.reset(self.window_width, self.window_height);
    }

    fn render(&mut self, c: Context, gl: &mut GlGraphics) {
        // clear everything
        graphics::clear(color::BLACK, gl);

        if self.showing_win_screen {
            // present the game over screen
            let mut text = graphics::Text::new(22);
            text.color = color::ORANGE;
            let player_number = if self.score_1 == self.winning_score {
                1
            } else {
                2
            };
            text.draw(&format!("Player {} wins! Click to continue", player_number),
                &mut self.font,
                &c.draw_state,
                c.trans(self.window_width / 2.0 - 200.0, self.window_height / 2.0).transform,
                gl);

            return;
        }

        // draw the ball
        let diameter = self.ball.radius * 2.0;
        Ellipse::new(color::VIOLET).draw(
            [self.ball.center.x - self.ball.radius, self.ball.center.y - self.ball.radius, diameter, diameter],
            &c.draw_state, c.transform, gl);

        // draw the left paddle
        Rectangle::new(color::WHITE).draw(
            [0.0, self.paddle_1_y - self.paddle_height / 2.0, self.paddle_width, self.paddle_height],
            &c.draw_state, c.transform, gl);

        // draw the right paddle
        Rectangle::new(color::WHITE).draw(
            [self.window_width - self.paddle_width, self.paddle_2_y - self.paddle_height / 2.0, self.paddle_width, self.paddle_height],
            &c.draw_state, c.transform, gl);

        // draw the score
        let mut text = graphics::Text::new(22);
        text.color = color::ORANGE;
        text.draw(&format!("Score: {}", self.score_1),
                  &mut self.font,
                  &c.draw_state,
                  c.trans(10.0, 20.0).transform,
                  gl);

        text.draw(&format!("Score: {}", self.score_2),
                  &mut self.font,
                  &c.draw_state,
                  c.trans(self.window_width / 2.0, 20.0).transform,
                  gl);
    }

    fn update(&mut self, dt: f64) {
        if self.showing_win_screen {
            // do nothing if the game is over
            return;
        }

        // move the ball
        self.ball.update(dt);

        // move the paddle
        self.paddle_1_y = self.mouse_y;

        // right paddle AI
        if self.ball.center.y > self.paddle_2_y {
            // if the ball is below our center, move down
            self.paddle_2_y += 200.0 * dt;
        } else if self.ball.center.y < self.paddle_2_y {
            // if the ball is above our center, move up
            self.paddle_2_y -= 200.0 * dt;
        }

        // check if the ball is touching the left paddle
        if self.ball.center.x - self.ball.radius < self.paddle_width
            && self.ball.center.y >= self.paddle_1_y - self.paddle_height / 2.0
            && self.ball.center.y <= self.paddle_1_y + self.paddle_height / 2.0 {

            // reflect the ball
            self.ball.speed.x = -self.ball.speed.x;
            let delta_y = self.ball.center.y - self.paddle_1_y;
            self.ball.speed.y = delta_y * 8.0;

        } else if self.ball.center.x - self.ball.radius <= 0.0 {
            // ball has hit the left side
            self.scored(2);
        }

        // check if the ball is touching the right paddle
        if self.ball.center.x + self.ball.radius > self.window_width - self.paddle_width
            && self.ball.center.y >= self.paddle_2_y - self.paddle_height / 2.0
            && self.ball.center.y <= self.paddle_2_y + self.paddle_height / 2.0 {

            // reflect the ball
            self.ball.speed.x = -self.ball.speed.x;
            let delta_y = self.ball.center.y - self.paddle_2_y;
            self.ball.speed.y = delta_y * 8.0;

        } else if self.ball.center.x + self.ball.radius >= self.window_width {
            // ball has hit the right side
            self.scored(1);
        }

        if self.ball.center.y + self.ball.radius >= self.window_height || self.ball.center.y - self.ball.radius <= 0.0 {
            // ball has hit the top or bottom
            self.ball.speed.y = -self.ball.speed.y;
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
            .samples(8)
            .vsync(true)
            .into();

    let mut gl = GlGraphics::new(opengl);

    let mut game = Game::new(width as f64, height as f64);

    for e in window.events().ups(60).max_fps(60) {
        match e {
            Event::Input(Input::Release(Button::Mouse(mouse))) => {
                if game.showing_win_screen {
                    match mouse {
                        MouseButton::Left => game.reset(),
                        _ => ()
                    }
                }
            }

            Event::Input(Input::Move(Motion::MouseCursor(_, y))) => {
                game.mouse_y = y;
            },

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

