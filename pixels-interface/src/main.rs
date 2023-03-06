use bevy_time::Time;
use clap::Parser;

use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use pixels_canvas::prelude::*;

use canvas::CanvasContainer;
use state::{
    ToolState,
    State
};

mod canvas;
mod input;
mod panel;
mod state;

#[derive(Parser)]
pub struct Args {
    /// Todo: Refresh token to connect the API
    refresh: String,
}

struct App {
    world: World,
    draw_schedule: Schedule,
    update_schedule: Schedule,
}

#[macroquad::main("Pixels Client")]
async fn main() {
    let mut app = App::new(Args::parse(), State::default());

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
            (canvas.height() * 2) as f32
        );
        state.camera_state.position = calculate_center(&canvas);

        let mut draw_schedule = Schedule::default();
        draw_schedule.add_stage("draw", SystemStage::single_threaded());

        let mut update_schedule = Schedule::default();
        update_schedule.add_stage(
            "update",
            SystemStage::parallel()
                .with_system(update_time)
                .with_system(update_camera)
        );

        canvas::register_systems(&mut world, &mut update_schedule, &mut draw_schedule);
        input::register_systems(&mut world, &mut update_schedule, &mut draw_schedule);
        panel::register_systems(&mut world, &mut update_schedule, &mut draw_schedule);

        world.insert_resource(CanvasContainer::new(canvas));
        world.insert_resource(Time::default());
        world.insert_resource(state);

        App {
            world,
            draw_schedule,
            update_schedule,
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

pub fn update_camera(mut state: ResMut<State>) {
    state.camera_state.instance = Camera2D {
        target: state.camera_state.position,
        zoom: calculate_zoom(state.camera_state.zoom),
        ..Default::default()
    };
    set_camera(&state.camera_state.instance);
}

pub fn calculate_zoom(factor: f32) -> Vec2 {
    vec2(
        1.0 / screen_width() * 2.0 * factor,
        -1.0 / screen_height() * 2.0 * factor,
    )
}

pub fn calculate_center(canvas: &Canvas) -> Vec2 {
    vec2(canvas.width() as f32, canvas.height() as f32) / 2.0
}

pub fn mouse_world_pos(camera: Camera2D) -> Vec2 {
    camera.screen_to_world(vec2(mouse_position().0, mouse_position().1))
}
