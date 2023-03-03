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
    focus: bool,
    color: [f32; 3],
    camera: Camera2D,
    position: Vec2,
    move_origin: Vec2,
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
    fn new(args: Args, mut canvas: Canvas, mut client: Client, mut state: State) -> Self {
        client.auth(args.refresh.clone()).expect("couldn't get access token");

        canvas.set_size(client.canvas_size().expect("couldn't get canvas size"));
        request_new_screen_size(
            (canvas.width()*2) as f32,
            (canvas.height()*2) as f32,
        );
        state.position = canvas.size_vec2() / vec2(2.0, 2.0);

        canvas.set_data(client.canvas_pixels().expect("couldn't get canvas pixels"));

        let mut world = World::new();

        let mut draw_schedule = Schedule::default();
        draw_schedule.add_stage("draw", SystemStage::single_threaded()
            .with_system(canvas::draw.label("canvas"))
            .with_system(draw_settings.after("canvas"))
        );

        let mut update_schedule = Schedule::default();
        update_schedule.add_stage("update", SystemStage::parallel()
            .with_system(system_draw)
            .with_system(system_move)
            .with_system(update_time)
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

pub fn system_draw(state: Res<State>, mut canvas: ResMut<Canvas>, client: Res<Client>) {
    if state.focus {
        return;
    }

    if is_mouse_button_pressed(MouseButton::Left) && is_key_down(KeyCode::C) {
        let pos = util::mouse_world_pos(state.camera);

        let color = util::rgb_f32_to_color(state.color);
        canvas.set_pixel(pos.x as u64, pos.y as u64, color);
        if let Err(_) = client.canvas_set_pixel(pos.x as u64, pos.y as u64, color) {
            println!("couldn't set pixel");
        }
    }
}

pub fn system_move(mut state: ResMut<State>) {
    if state.focus {
        return;
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        state.move_origin = util::mouse_world_pos(state.camera);
    } else if is_mouse_button_down(MouseButton::Left) {
        let pos = util::mouse_world_pos(state.camera);
        let origin = state.move_origin;

        if pos.distance(origin) > 1.0 {
            state.position += origin - pos;
        }
    }
}

pub fn update_camera(mut state: ResMut<State>) {
    state.camera = Camera2D {
        target: state.position,
        zoom: util::calculate_zoom(state.zoom),
        ..Default::default()
    };
    set_camera(&state.camera);
}

pub fn draw_settings(mut state: ResMut<State>) {
    egui_macroquad::ui(|ctx| {
        state.focus = ctx.is_pointer_over_area();

        egui::Window::new("my_left_panel").show(ctx, |ui| {
            ui.label("color:");
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
            zoom: 3.0,
            focus: false,
            color: [1.0, 1.0, 1.0],
            camera: Camera2D::default(),
            position: vec2(0.0, 0.0),
            move_origin: vec2(0.0, 0.0),
        }
    }
}
