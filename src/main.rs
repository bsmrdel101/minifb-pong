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
  let player_1_score = 0;
  let player_2_score = 0;
  let mut ball = Ball::new(30, 30);
  let mut paddle_1 = Paddle::new(20, 50);

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

    for (i, pixel) in buffer.iter_mut().enumerate() {
      *pixel = BG;
      let (x, y) = (i % WIDTH, (i / WIDTH));
      
      if started &&
        x >= ball.x && x <= ball.x + ball.w &&
        y >= ball.y && y <= ball.y + ball.h
      {
        *pixel = ball.color;
      }

      if
        x >= paddle_1.x && x <= paddle_1.x + paddle_1.w &&
        y >= paddle_1.y && y <= paddle_1.y + paddle_1.h
      {
        *pixel = paddle_1.color;
      }
    }

    if started {
      ball.y = (ball.y as i32 + ball.y_vel) as usize;
      ball.x = (ball.x as i32 + ball.x_vel) as usize;


      if ball.x <= 0 || ball.x >= WIDTH {
        ball.respawn();
      }

      if ball.y >= HEIGHT - ball.h {
        ball.y_vel = -ball.y_vel;
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
  pub fn new(w: usize, h: usize) -> Self {
    Self {
      w,
      h,
      x: 5,
      y: HEIGHT / 2,
      color: 0xffffff
    }
  }
}
