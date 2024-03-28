use std::env;

use learn_vk::vulkan::engine::VulkanEngine;
use learn_vk::window::get_event_loop;
use learn_vk::MyError;

use sllog::*;
use winit::event::MouseScrollDelta;
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
    let mut minimized = false;

    // Window
    let event_loop = get_event_loop();
    let window = WindowBuilder::new()
        .with_title("Vulkan Tutorial (Rust)")
        .with_inner_size(LogicalSize::new(1280, 720))
        .build(&event_loop)?;

    let vk_engine = unsafe { VulkanEngine::new(window)? };

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::MainEventsCleared if !minimized => {
                unsafe { vk_engine.render().unwrap() }
            }
            Event::WindowEvent {event, .. } => {
                match event { WindowEvent::CloseRequested => { 
                        *control_flow = ControlFlow::Exit;
                        unsafe { vk_engine.destroy(); }
                    },
                    WindowEvent::Resized(size) => {
                        if size.width == 0 || size.height == 0 {
                            minimized = true;
                            std::thread::sleep(std::time::Duration::from_millis(100));
                        }
                        else {
                            minimized = false;
                            app.resized = true;
                        }
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    });
}
