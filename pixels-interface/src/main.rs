use bevy_time::{Time, Timer, TimerMode};
use std::time::Duration;

use macroquad::prelude::*;
use bevy_ecs::prelude::*;
use egui_macroquad::egui;
use clap::Parser;
use egui_macroquad::egui::Widget;

use pixels_canvas::prelude::*;

use pixels_util::color::Color;
use canvas::{
    CanvasContainer,
    CanvasTimer
};

mod canvas;

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
    cooldown: f32,
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
        State::default()
    );

    loop {
        app.update();
        app.draw();
        next_frame().await
    }
}

impl App {
    fn new(args: Args, mut state: State) -> Self {
        let canvas = Canvas::new(args.refresh);
        let mut world = World::new();

        request_new_screen_size(
            (canvas.width() * 2) as f32,
            (canvas.height() * 2) as f32,
        );
        state.position = calculate_center(&canvas);

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
            .with_system(update_cooldown)
        );

        world.insert_resource(CanvasContainer::new(canvas));
        world.insert_resource(state);

        world.insert_resource(Time::default());
        world.insert_resource(CanvasTimer::new(Timer::new(
            Duration::from_secs(5), TimerMode::Repeating
        )));

        App {
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

pub fn update_cooldown(mut state: ResMut<State>, container: ResMut<CanvasContainer>){
    state.cooldown = container.canvas.get_cooldown().remaining();
}

pub fn update_input(mut state: ResMut<State>, mut container: ResMut<CanvasContainer>) {
    if is_key_pressed(KeyCode::Z) {
        state.focus = !state.focus
    }

    if !state.focus {
        return;
    }

    let pos = mouse_world_pos(state.camera);

    state.zoom = (state.zoom + mouse_wheel().1/120.0).clamp(1.0, 10.0);

    if is_mouse_button_pressed(MouseButton::Left) {
        state.move_origin = pos;

        if is_key_down(KeyCode::C) {
            let color = Color::from(state.color);
            if let Err(e) = container.canvas.set_pixel(pos.x as u64, pos.y as u64, color) {
                match e {
                    CanvasError::ClientError => {
                        panic!("couldn't set pixel");
                    }
                    CanvasError::Cooldown(cooldown) => {
                        println!("please wait cooldown to end: {}", cooldown.remaining());
                    }
                }
            }
        }
        if is_key_down(KeyCode::X) {
            state.color = container.canvas.pixel(pos.x as u64, pos.y as u64).as_array();
        }
    } else if is_mouse_button_down(MouseButton::Left) {
        let origin = state.move_origin;
        state.position += origin - pos;
    }
}

pub fn update_camera(mut state: ResMut<State>) {
    state.camera = Camera2D {
        target: state.position,
        zoom: calculate_zoom(state.zoom),
        ..Default::default()
    };
    set_camera(&state.camera);
}

pub fn draw_settings(mut state: ResMut<State>) {
    if state.focus { return; }

    egui_macroquad::ui(|ctx| {
        egui::SidePanel::left("settings").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("");
                ui.label("Pixels Client Settings");
                ui.label("");
                ui.label("color:");
                ui.color_edit_button_rgb(&mut state.color);
                ui.label("");
                ui.label(format!("cooldown: {}", state.cooldown));
            })
        });
    });

    egui_macroquad::draw();
}

impl Default for State {
    fn default() -> Self {
        State {
            cooldown: 0.0,
            zoom: 3.0,
            focus: true,
            color: [1.0, 1.0, 1.0],
            camera: Camera2D::default(),
            position: vec2(0.0, 0.0),
            move_origin: vec2(0.0, 0.0),
        }
    }
}

pub fn calculate_zoom(factor: f32) -> Vec2 {
    vec2(
        1.0 / screen_width() * 2.0 * factor,
        -1.0 / screen_height() * 2.0 * factor,
    )
}

pub fn calculate_center(canvas: &Canvas) -> Vec2 {
    vec2(
        canvas.width() as f32,
        canvas.height() as f32
    ) / 2.0
}

pub fn mouse_world_pos(camera: Camera2D) -> Vec2 {
    camera.screen_to_world(
        vec2(mouse_position().0, mouse_position().1)
    )
}
