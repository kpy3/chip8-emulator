use winit::event::{Event, VirtualKeyCode};
use crate::{keypad, font};
use crate::keypad::Keypad;
use winit_input_helper::WinitInputHelper;
use crate::chip8::Chip8;
use pixels::{SurfaceTexture, Pixels};
use std::time::Duration;
use winit::event_loop::{ControlFlow, EventLoop};
use std::thread;
use winit::dpi::{PhysicalSize, LogicalSize, LogicalPosition};

const KEYPAD_MAP: [(usize, VirtualKeyCode); keypad::KEYPAD_SIZE] = [
    (0, VirtualKeyCode::X),
    (1, VirtualKeyCode::Key1),
    (2, VirtualKeyCode::Key2),
    (3, VirtualKeyCode::Key3),
    (4, VirtualKeyCode::Q),
    (5, VirtualKeyCode::W),
    (6, VirtualKeyCode::E),
    (7, VirtualKeyCode::A),
    (8, VirtualKeyCode::S),
    (9, VirtualKeyCode::D),
    (10, VirtualKeyCode::Z),
    (11, VirtualKeyCode::C),
    (12, VirtualKeyCode::Key4),
    (13, VirtualKeyCode::R),
    (14, VirtualKeyCode::F),
    (15, VirtualKeyCode::V)
];

pub fn run(data: &[u8], sleep_duration: u64) {
    let mut chip8 = Chip8::new();
    chip8.load_fontset(&font::DEFAULT_FONTSET);
    chip8.load_rom(&data);

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, surface, width, height, mut _hidpi_factor) =
        create_window("Chip8 Emulator", &event_loop);
    let surface_texture = SurfaceTexture::new(width, height, surface);
    let mut pixels = Pixels::new(64, 32, surface_texture).unwrap();
    let sleep_duration = Duration::from_millis(sleep_duration);

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

            let keypad = keypad(&input);

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

fn keypad(input: &WinitInputHelper) -> Keypad {
    let mut keypad = Keypad::new();
    for (key, key_code) in &KEYPAD_MAP {
        if input.key_pressed(*key_code) || input.key_held(*key_code) {
            keypad.press(*key);
        }
    }
    keypad
}
