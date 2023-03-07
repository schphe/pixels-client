use bevy_time::Time;
use clap::Parser;

use rfd::{
    FileDialog,
    MessageDialog,
    MessageButtons
};

use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use pixels_canvas::prelude::*;

use canvas::CanvasContainer;
use state::{State, ToolType};
use pixels_canvas::image::Image;

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

fn main() {
    let path = get_image_path();
    macroquad::Window::new("Pixels Client", entry(path));
}

async fn entry(image: Option<Image>) {
    let mut app = App::new(
        Args::parse(),
        State::new(image)
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
            (canvas.height() * 2) as f32
        );
        state.camera_state.position = calculate_center(&canvas);

        let mut draw_schedule = Schedule::default();
        let mut update_schedule = Schedule::default();

        update_schedule.add_systems((
            update_time,
            update_camera
        ));

        canvas::register_systems(
            canvas,
            &mut world,
            &mut update_schedule,
            &mut draw_schedule,
        );
        input::register_systems(&mut update_schedule);

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
        panel::draw(&mut self.world);
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

fn get_image_path() -> Option<Image> {
    let select = MessageDialog::new()
        .set_buttons(MessageButtons::YesNo)
        .set_description("would you like to select an image to paste?")
        .show();
    if !select {
        return None
    }

    let path = FileDialog::new()
        .add_filter("PNG Image", &["png"])
        .add_filter("JPEG Image", &["jpg", "jpeg"])
        .set_directory("~")
        .pick_file();
    path.map(|p| Image::new(p))
}
