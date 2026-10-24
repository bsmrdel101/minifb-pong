use std::{thread::sleep, time, vec};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use rand::RngExt;
use crate::text::Text;

mod text;


const WIDTH: usize = 640 * 2;
const HEIGHT: usize = 360 * 2;
const BG: u32 = 0x000000;

struct Ball {
  w: usize,
  h: usize,
  x: usize,
  y: usize,
  x_vel: i32,
  y_vel: i32,
  color: u32
}

struct Paddle {
  w: usize,
  h: usize,
  x: usize,
  y: usize,
  color: u32
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
  a + (b - a) * t
}

fn main() {
  let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

  let mut window = Window::new(
    "Pong",
    WIDTH,
    HEIGHT,
    WindowOptions {
      resize: true,
      ..WindowOptions::default()
    }
  ).unwrap();

  window.set_target_fps(60);


  let mut started = false;
  let start_text = Text::new(1280, 720, 6);
  let score_text = Text::new(1280, 720, 4);
  let mut player_1_score = 0;
  let mut player_2_score = 0;
  let mut ball = Ball::new(30, 30);
  let mut paddle_1 = Paddle::new(5, HEIGHT / 2, 20, 50);
  let mut paddle_2 = Paddle::new(WIDTH - 5 - 20, HEIGHT / 2, 20, 50);

  while window.is_open() && !window.is_key_down(Key::Escape) {
    if window.get_mouse_down(MouseButton::Left) {
      started = true;
    }

    if let Some((_, y)) = window.get_mouse_pos(MouseMode::Discard) {
      paddle_1.y = y as usize;
      if paddle_1.y > HEIGHT - paddle_1.h {
        paddle_1.y = HEIGHT - paddle_1.h;
      }
    }

    paddle_2.y = lerp(ball.y as f32 - paddle_2.h as f32 / 2.0, paddle_2.y as f32, 0.67) as usize;

    for (i, pixel) in buffer.iter_mut().enumerate() {
      *pixel = BG;
      let (x, y) = (i % WIDTH, (i / WIDTH));
      
      ball.render(&x, &y, pixel, started);
      paddle_1.render(&x, &y, pixel);
      paddle_2.render(&x, &y, pixel);
    }

    if started {
      let mut new_x = ball.x as i32 + ball.x_vel;
      let new_y = ball.y as i32 + ball.y_vel;

      new_x = paddle_1.collide(&mut new_x, &new_y, &mut ball);
      new_x = paddle_2.collide(&mut new_x, &new_y, &mut ball);

      if new_x >= WIDTH as i32 {
        player_1_score += 1;
        ball.respawn();
      } else if new_x + (ball.w as i32) <= 0 {
        player_2_score += 1;
        ball.respawn();
      } else {
        ball.x = new_x as usize;
      }

      if new_y + (ball.h as i32) >= (HEIGHT as i32) || new_y - (ball.h as i32) <= 0 {
        ball.y_vel = -ball.y_vel;
      } else {
        ball.y = new_y as usize;
      }

      score_text.draw(&mut buffer, (WIDTH / 2 - 75, 10), &format!("{} | {}", player_1_score, player_2_score));
    } else {
      start_text.draw(&mut buffer, (WIDTH / 2 - 330, HEIGHT / 2), "Click to start");
    }

    window
      .update_with_buffer(&buffer, WIDTH, HEIGHT)
      .unwrap();
  }
}


impl Ball {
  pub fn new(w: usize, h: usize) -> Self {
    let mut ball = Self {
      w,
      h,
      x: WIDTH / 2 - w,
      y: HEIGHT / 2,
      x_vel: 0,
      y_vel: 0,
      color: 0xffffff
    };
    ball.respawn();
    ball
  }

  pub fn render(&self, x: &usize, y: &usize, pixel: &mut u32, started: bool) {
    if started &&
      *x >= self.x && *x <= self.x + self.w &&
      *y >= self.y && *y <= self.y + self.h
    {
      *pixel = self.color;
    }
  }

  pub fn respawn(&mut self) {
    sleep(time::Duration::from_secs_f32(0.5));
    let side = rand::rng().random_range(0..=1);
    let mut x_vel = rand::rng().random_range(10..30);
    if side == 0 {
      x_vel = -x_vel;
    }

    self.x = WIDTH / 2 - self.w;
    self.x_vel = x_vel;
  }
}

impl Paddle {
  pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
    Self {
      w,
      h,
      x,
      y,
      color: 0xffffff
    }
  }

  pub fn render(&self, x: &usize, y: &usize, pixel: &mut u32) {
    if
      *x >= self.x && *x <= self.x + self.w &&
      *y >= self.y && *y <= self.y + self.h
    {
      *pixel = self.color;
    }
  }

  pub fn collide(&self, new_x: &mut i32, new_y: &i32, ball: &mut Ball) -> i32 {
    if
      *new_x >= (self.x as i32) && *new_x <= (self.x as i32) + (self.w as i32) &&
      *new_y + (ball.h as i32) >= (self.y as i32) && *new_y <= (self.y as i32) + (self.h as i32)
    {
      *new_x = ball.x as i32 + -ball.x_vel;
      ball.x_vel = -ball.x_vel;
      ball.y_vel = ((self.y as i32) - (ball.y as i32)) / 2;
    }
    *new_x
  }
}
