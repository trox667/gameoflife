use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::{
    event::{Event, VirtualKeyCode},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

// https://crates.io/crates/pixels
// https://github.com/parasyte/pixels/blob/master/examples/minimal-winit/src/main.rs

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

struct Cell {
    x: u32,
    y: u32,
    size: u32,
}

impl Cell {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            size: 4,
        }
    }

    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as u32;
            let y = (i / WIDTH as usize) as u32;

            if x == self.x && y == self.y {
                pixel.copy_from_slice(&[0xff, 0x00, 0x00, 0xff])
            } else {
                pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff])
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let cell = Cell::new();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut hidpi_factor = window.scale_factor();
    let mut pixels = {
        let surface = Surface::create(&window);
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, surface);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            cell.draw(pixels.get_frame());
            pixels.render().unwrap()
        }
        if input.update(event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // Adjust high DPI factor
            if let Some(factor) = input.scale_factor_changed() {
                hidpi_factor = factor;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }
            window.request_redraw();
        }
    });
}
