mod keypad;
mod stack;
mod display;
mod memory;

use std::env;
use crate::chip8::Chip8;
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use pixels::{SurfaceTexture, Pixels};

mod chip8;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let data = std::fs::read(filename).map_err(|e| format!("{}", e))?;
    let mut chip8 = Chip8::new();
    chip8.load_fontset(&default_fontset());
    chip8.load_rom(&data);
    run(chip8);
    Ok(())
}

fn run(mut chip8: Chip8) {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, surface, width, height, mut _hidpi_factor) =
        create_window("Chip8 Emulator", &event_loop);
    let surface_texture = SurfaceTexture::new(width, height, surface);
    let mut pixels = Pixels::new(64, 32, surface_texture).unwrap();

    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            chip8.draw(pixels.get_frame());
            if pixels
                .render()
                // .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Key1) {
                chip8.key_pressed(1);
            }

            if input.key_pressed(VirtualKeyCode::Key2) {
                chip8.key_pressed(2);
            }

            if input.key_pressed(VirtualKeyCode::Key3) {
                chip8.key_pressed(3);
            }

            if input.key_pressed(VirtualKeyCode::Q) {
                chip8.key_pressed(4);
            }

            if input.key_pressed(VirtualKeyCode::W) {
                chip8.key_pressed(5);
            }

            if input.key_pressed(VirtualKeyCode::E) {
                chip8.key_pressed(6);
            }

            if input.key_pressed(VirtualKeyCode::R) {
                chip8.key_pressed(0xD);
            }

            if input.key_pressed(VirtualKeyCode::A) {
                chip8.key_pressed(7);
            }

            if input.key_pressed(VirtualKeyCode::S) {
                chip8.key_pressed(8);
            }

            if input.key_pressed(VirtualKeyCode::D) {
                chip8.key_pressed(9);
            }

            if input.key_pressed(VirtualKeyCode::F) {
                chip8.key_pressed(0xE);
            }

            if input.key_pressed(VirtualKeyCode::Z) {
                chip8.key_pressed(0xA);
            }

            if input.key_pressed(VirtualKeyCode::X) {
                chip8.key_pressed(0);
            }

            if input.key_pressed(VirtualKeyCode::C) {
                chip8.key_pressed(0xB);
            }

            if input.key_pressed(VirtualKeyCode::V) {
                chip8.key_pressed(0xF);
            }

            if input.key_released(VirtualKeyCode::Key1) {
                chip8.key_released(1);
            }

            if input.key_released(VirtualKeyCode::Key2) {
                chip8.key_released(2);
            }

            if input.key_released(VirtualKeyCode::Key3) {
                chip8.key_released(3);
            }

            if input.key_released(VirtualKeyCode::Q) {
                chip8.key_released(4);
            }

            if input.key_released(VirtualKeyCode::W) {
                chip8.key_released(5);
            }

            if input.key_released(VirtualKeyCode::E) {
                chip8.key_released(6);
            }

            if input.key_released(VirtualKeyCode::R) {
                chip8.key_released(0xD);
            }

            if input.key_released(VirtualKeyCode::A) {
                chip8.key_released(7);
            }

            if input.key_released(VirtualKeyCode::S) {
                chip8.key_released(8);
            }

            if input.key_released(VirtualKeyCode::D) {
                chip8.key_released(9);
            }

            if input.key_released(VirtualKeyCode::F) {
                chip8.key_released(0xE);
            }

            if input.key_released(VirtualKeyCode::Z) {
                chip8.key_released(0xA);
            }

            if input.key_released(VirtualKeyCode::X) {
                chip8.key_released(0);
            }

            if input.key_released(VirtualKeyCode::C) {
                chip8.key_released(0xB);
            }

            if input.key_released(VirtualKeyCode::V) {
                chip8.key_released(0xF);
            }

        }

        // Adjust high DPI factor
        if let Some(factor) = input.scale_factor_changed() {
            _hidpi_factor = factor;
        }

        // Resize the window
        if let Some(size) = input.window_resized() {
            pixels.resize(size.width, size.height);
        }

        chip8.tick();
        window.request_redraw();
    })

}

fn create_window(
    title: &str,
    event_loop: &EventLoop<()>,
) -> (winit::window::Window, pixels::wgpu::Surface, u32, u32, f64) {
    // Create a hidden window so we can estimate a good default window size
    let window = winit::window::WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .build(&event_loop)
        .unwrap();
    let hidpi_factor = window.scale_factor();

    // Get dimensions
    let width = 64.0;
    let height = 32.0;
    let (monitor_width, monitor_height) = {
        let size = window.current_monitor().size();
        (
            size.width as f64 / hidpi_factor,
            size.height as f64 / hidpi_factor,
        )
    };
    let scale = (monitor_height / height * 2.0 / 3.0).round();

    // Resize, center, and display the window
    let min_size = PhysicalSize::new(width, height).to_logical::<f64>(hidpi_factor);
    let default_size = LogicalSize::new(width * scale, height * scale);
    let center = LogicalPosition::new(
        (monitor_width - width * scale) / 2.0,
        (monitor_height - height * scale) / 2.0,
    );
    window.set_inner_size(default_size);
    window.set_min_inner_size(Some(min_size));
    window.set_outer_position(center);
    window.set_visible(true);

    let surface = pixels::wgpu::Surface::create(&window);
    let size = default_size.to_physical::<f64>(hidpi_factor);

    (
        window,
        surface,
        size.width.round() as u32,
        size.height.round() as u32,
        hidpi_factor,
    )
}


fn default_fontset() -> Vec<u8> {
    vec![
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
    ]
}