use std::f32::consts::PI;

use raylib::math::Rectangle;
use raylib::{color::Color, math::Vector2, prelude::RaylibDraw};
use raylib::ffi::KeyboardKey;

struct Window;

impl Window {
	const WIDTH: i32 = 800;
	const HEIGHT: i32 = 480;
}

struct Paddle {
	y: i32
}

impl Paddle {
	const PADDING: i32 = 50;
	const WIDTH: i32 = 10;
	const HEIGHT: i32 = 100;
	const SPEED: i32 = 4;
}

#[derive(Clone, Copy)]
struct Ball {
	speed: f32,
	angle: f32,
	position: Vector2
}

impl Ball {
	const RADIUS: i32 = 10;
	fn default() -> Ball {
			Ball {
			position: Vector2::new((Window::WIDTH/2) as f32, (Window::HEIGHT/2) as f32),
			speed: 3.0,
			angle: 0.0
		}
	}
}

fn check_collision_recs(a: Rectangle, b: Rectangle) -> bool {
	a.x < b.x + b.width &&
	a.x + a.width > b.x &&
	a.y < b.y + b.height &&
	a.y + a.height > b.y
}

fn bounce_ball(ball: &mut Ball, origin_y: i32) {
	ball.speed += 0.3;
	ball.angle += 180.0 - (((origin_y + Paddle::HEIGHT) as f32 / 2.0 - ball.position.y) * 0.4);
}

fn is_x_outside_screen(x: f32, width: i32) -> bool {
	x < 0.0 || x + width as f32 > Window::WIDTH as f32
}

fn main() {
	let (mut rl, thread) = raylib::init()
	.msaa_4x()
	.vsync()
	.size(Window::WIDTH, Window::HEIGHT)
	.title("PING PONG")
	.build();

	let mut p1 = Paddle { y: Window::HEIGHT / 2 - Paddle::HEIGHT / 2 };
	let mut p2 = Paddle { ..p1 }; // same as p1

	let mut ball = Ball::default();

	rl.set_target_fps(60);

	let bg = Color::new(32, 32, 54, 255);

	while !rl.window_should_close() {
		//Game logic comes first to avoid error
		//Player
		{
			if rl.is_key_down(KeyboardKey::KEY_UP) { p1.y -= Paddle::SPEED; }
			if rl.is_key_down(KeyboardKey::KEY_DOWN) { p1.y += Paddle::SPEED; }

			p1.y = i32::max(p1.y, 0);
			p1.y = i32::min(p1.y, Window::HEIGHT - Paddle::HEIGHT);
		}

		//Bot
		{
			if ball.position.y < (p2.y + 20) as f32 {
				p2.y -= Paddle::SPEED;
			}
			else if ball.position.y > (p2.y + Paddle::HEIGHT - 20) as f32 {
				p2.y += Paddle::SPEED;
			}

			p2.y = i32::max(p2.y, 0);
			p2.y = i32::min(p2.y, Window::HEIGHT - Paddle::HEIGHT);
		}

		//Ball
		{
			let ball_radius = Ball::RADIUS as f32;
			let window_height = Window::HEIGHT as f32;

			if ball.position.y < 0.0 || ball.position.y + ball_radius > window_height { ball.angle *= -1.0 }

			let ballrect = Rectangle::new(ball.clone().position.x as f32, ball.clone().position.y as f32, Ball::RADIUS as f32, Ball::RADIUS as f32);
			let p1rect = Rectangle::new(Paddle::PADDING as f32, p1.y as f32, Paddle::WIDTH as f32, Paddle::HEIGHT as f32);
			let p2rect = Rectangle {x: (Window::WIDTH - Paddle::WIDTH - Paddle::PADDING) as f32, y: p2.y as f32, ..p1rect};

			if check_collision_recs(p1rect, ballrect) { bounce_ball(&mut ball, p1.y) }
			if check_collision_recs(p2rect, ballrect) { bounce_ball(&mut ball, p2.y) }

			ball.position.x += ball.speed * f32::cos(ball.angle * (PI / 180.0));
			ball.position.y += ball.speed * f32::sin(ball.angle * (PI / 180.0));

			if is_x_outside_screen(ball.position.x, Ball::RADIUS) {
				ball = Ball::default()
			}
		}

		//Render
		{
			let mut render = rl.begin_drawing(&thread);
			render.clear_background(bg);

			render.draw_rectangle(Paddle::PADDING, p1.y, Paddle::WIDTH, Paddle::HEIGHT, Color::WHITE);
			render.draw_rectangle(Window::WIDTH - Paddle::WIDTH - Paddle::PADDING, p2.y, Paddle::WIDTH, Paddle::HEIGHT, Color::WHITE);
			render.draw_rectangle(ball.position.x as i32, ball.position.y as i32, Ball::RADIUS, Ball::RADIUS, Color::WHITE);
		}
	}
}