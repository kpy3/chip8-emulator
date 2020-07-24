mod keypad;
mod stack;
mod display;
mod memory;

use std::{env, thread};
use crate::chip8::Chip8;
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use pixels::{SurfaceTexture, Pixels};
use crate::keypad::Keypad;
use std::time::Duration;

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
    let sleep_duration = Duration::from_millis(2);
    let mut keypad = Keypad::new();

    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            if chip8.display_changed() {
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
        }

        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Key1) {
                keypad.press(1);
            }

            if input.key_pressed(VirtualKeyCode::Key2) {
                keypad.press(2);
            }

            if input.key_pressed(VirtualKeyCode::Key3) {
                keypad.press(3);
            }

            if input.key_pressed(VirtualKeyCode::Key4) {
                keypad.press(0xC);
            }

            if input.key_pressed(VirtualKeyCode::Q) {
                keypad.press(4);
            }

            if input.key_pressed(VirtualKeyCode::W) {
                keypad.press(5);
            }

            if input.key_pressed(VirtualKeyCode::E) {
                keypad.press(6);
            }

            if input.key_pressed(VirtualKeyCode::R) {
                keypad.press(0xD);
            }

            if input.key_pressed(VirtualKeyCode::A) {
                keypad.press(7);
            }

            if input.key_pressed(VirtualKeyCode::S) {
                keypad.press(8);
            }

            if input.key_pressed(VirtualKeyCode::D) {
                keypad.press(9);
            }

            if input.key_pressed(VirtualKeyCode::F) {
                keypad.press(0xE);
            }

            if input.key_pressed(VirtualKeyCode::Z) {
                keypad.press(0xA);
            }

            if input.key_pressed(VirtualKeyCode::X) {
                keypad.press(0);
            }

            if input.key_pressed(VirtualKeyCode::C) {
                keypad.press(0xB);
            }

            if input.key_pressed(VirtualKeyCode::V) {
                keypad.press(0xF);
            }

            if input.key_released(VirtualKeyCode::Key1) {
                keypad.release(1);
            }

            if input.key_released(VirtualKeyCode::Key2) {
                keypad.release(2);
            }

            if input.key_released(VirtualKeyCode::Key3) {
                keypad.release(3);
            }

            if input.key_released(VirtualKeyCode::Key4) {
                keypad.release(0xC);
            }

            if input.key_released(VirtualKeyCode::Q) {
                keypad.release(4);
            }

            if input.key_released(VirtualKeyCode::W) {
                keypad.release(5);
            }

            if input.key_released(VirtualKeyCode::E) {
                keypad.release(6);
            }

            if input.key_released(VirtualKeyCode::R) {
                keypad.release(0xD);
            }

            if input.key_released(VirtualKeyCode::A) {
                keypad.release(7);
            }

            if input.key_released(VirtualKeyCode::S) {
                keypad.release(8);
            }

            if input.key_released(VirtualKeyCode::D) {
                keypad.release(9);
            }

            if input.key_released(VirtualKeyCode::F) {
                keypad.release(0xE);
            }

            if input.key_released(VirtualKeyCode::Z) {
                keypad.release(0xA);
            }

            if input.key_released(VirtualKeyCode::X) {
                keypad.release(0);
            }

            if input.key_released(VirtualKeyCode::C) {
                keypad.release(0xB);
            }

            if input.key_released(VirtualKeyCode::V) {
                keypad.release(0xF);
            }
            // Adjust high DPI factor
            if let Some(factor) = input.scale_factor_changed() {
                _hidpi_factor = factor;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            chip8.tick(keypad);
            window.request_redraw();
            thread::sleep(sleep_duration);
        }
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