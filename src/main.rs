use bevy::input::InputPlugin;
use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;
use bevy::log::*;
use bevy::render::RenderPlugin;
use bevy::window::WindowPlugin;
use bevy::winit::WinitPlugin;

const ONE_OVER_60: u32 = 16_666_667;


fn log_start() {
    info!("App started");
}

fn log_update() {
    info!("App updated");
}

fn main() {
    let _dp = DefaultPlugins;
    let _c = Color::rgb(0.4, 0.4, 0.4);

    App::new()
    // .add_plugins(DefaultPlugins)
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin::default())
        .add_plugin(InputPlugin::default())
        .add_plugin(WindowPlugin::default())
        .add_plugin(WinitPlugin::default())
        .add_plugin(RenderPlugin::default())
        // .insert_resource(LogSettings {
        //     level: bevy::log::Level::TRACE,
        //     filter: "".to_string(),
        // })
        .add_startup_system(log_start)
        .add_system(log_update)
        .add_system(exit_on_esc_system)
        .run();
}
