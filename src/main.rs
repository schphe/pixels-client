use bevy_time::{Time, Timer, TimerMode};
use std::time::Duration;

use macroquad::prelude::*;
use bevy_ecs::prelude::*;
use clap::Parser;

use canvas::Canvas;
use client::Client;

mod canvas;
mod client;
mod util;

#[derive(Parser)]
pub struct Args {
    refresh: String,
}

struct App {
    world: World,
    draw_schedule: Schedule,
    update_schedule: Schedule,
}

#[derive(Resource)]
pub struct State {
    zoom: f32,
    color: [f32; 3],
    camera: Camera2D
}

#[macroquad::main("Pixels Client")]
async fn main() {
    let mut app = App::new(
        Args::parse(),
        Canvas::new(),
        Client::new(),
        State::default()
    );

    loop {
        app.update();
        app.draw();
        next_frame().await
    }
}

impl App {
    fn new(args: Args, mut canvas: Canvas, mut client: Client, state: State) -> Self {
        client.auth(args.refresh.clone()).expect("couldn't get access token");

        canvas.set_size(client.canvas_size().expect("couldn't get canvas size"));
        request_new_screen_size(
            (canvas.width()*2) as f32,
            (canvas.height()*2) as f32,
        );
        canvas.set_data(client.canvas_pixels().expect("couldn't get canvas pixels"));

        let mut world = World::new();

        let mut draw_schedule = Schedule::default();
        draw_schedule.add_stage("draw", SystemStage::single_threaded()
            .with_system(canvas::draw.label("canvas"))
            .with_system(draw_settings.after("canvas"))
        );

        let mut update_schedule = Schedule::default();
        update_schedule.add_stage("update", SystemStage::parallel()
            .with_system(update_time)
            .with_system(update_input)
            .with_system(update_camera)
            .with_system(canvas::update)
        );

        world.insert_resource(canvas);
        world.insert_resource(client);
        world.insert_resource(state);

        world.insert_resource(Time::default());
        world.insert_resource(canvas::CanvasTimer(Timer::new(
            Duration::from_secs(5), TimerMode::Repeating
        )));

        return App {
            world, draw_schedule, update_schedule
        }
    }

    fn update(&mut self) {
        self.update_schedule.run(&mut self.world);
    }

    fn draw(&mut self) {
        clear_background(DARKGRAY);
        self.draw_schedule.run(&mut self.world);
    }
}

pub fn update_time(mut time: ResMut<Time>) {
    time.update()
}

pub fn update_input(state: Res<State>, mut canvas: ResMut<Canvas>, client: Res<Client>) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let pos = state.camera.screen_to_world(
            vec2(mouse_position().0, mouse_position().1)
        );

        let color = util::rgb_f32_to_color(state.color);
        canvas.set_pixel(pos.x as u64, pos.y as u64, color);
        if let Err(_) = client.canvas_set_pixel(pos.x as u64, pos.y as u64, color) {
            println!("couldn't set pixel");
        }
    }
}

pub fn update_camera(mut state: ResMut<State>, canvas: Res<Canvas>) {
    state.camera = Camera2D {
        target: canvas.size_vec2() / vec2(2.0, 2.0),
        zoom: util::calculate_zoom(state.zoom),
        ..Default::default()
    };
    set_camera(&state.camera);
}

pub fn draw_settings(mut state: ResMut<State>) {
    egui_macroquad::ui(|ctx| {
        egui::Window::new("settings")
            .show(ctx, |ui| {
                ui.label("cursor color:");
                ui.color_edit_button_rgb(&mut state.color);
                ui.label("zoom:");
                ui.add(egui::Slider::new(&mut state.zoom, 1.0..=10.0));
            });
    });

    egui_macroquad::draw();
}

impl Default for State {
    fn default() -> Self {
        return State{
            zoom: 1.0,
            color: [1.0, 1.0, 1.0],
            camera: Camera2D::default()
        }
    }
}
