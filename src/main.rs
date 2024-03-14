#![allow(
    dead_code,
    unused_variables,
    clippy::unnecessary_wraps
)]

use std::env;

use learn_vk::input::Input;
use learn_vk::{application::App, window::get_event_loop};
use learn_vk::MyError;

use sllog::info;
use winit::{
    dpi::LogicalSize,
    event::{
        Event,
        WindowEvent
    },
    event_loop::ControlFlow,
    window::WindowBuilder,
};

fn main() -> Result<(), MyError> {
    env::set_var("LOG", "4");

    // Window
    let event_loop = get_event_loop();
    let window = WindowBuilder::new()
        .with_title("Vulkan Tutorial (Rust)")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    // App
    let mut app = unsafe { App::create(&window)? };
    let mut destroying = false;
    let mut minimized = false;
    
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::MainEventsCleared if !destroying && !minimized => {
                unsafe { app.render(&window).unwrap() }
            }
            Event::WindowEvent {event, .. } => {
                match event {
                    WindowEvent::CloseRequested => {
                        destroying = true;
                        *control_flow = ControlFlow::Exit;
                        unsafe { app.destroy(); }
                    },
                    WindowEvent::Resized(size) => {
                        if size.width == 0 || size.height == 0 {
                            minimized = true;
                        }
                        else {
                            minimized = false;
                            app.resized = true;
                        }
                    },
                    WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {
                        app.input.set_key_state(input.virtual_keycode, input.state);
                    },
                    WindowEvent::MouseInput { device_id, state, button, modifiers } => {
                        app.input.set_mouse_state(button, state);
                    },
                    WindowEvent::CursorMoved { device_id, position, modifiers } => {
                        app.input.set_mouse_position(position.x as f32, position.y as f32);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });
}

