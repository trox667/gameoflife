use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};
use rand::prelude::*;
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

#[derive(Eq, PartialEq, Copy, Clone)]
enum CellStatus {
    Alive,
    Dead,
}

struct Cell {
    status: CellStatus,
}

impl Cell {
    fn new(status: CellStatus) -> Self {
        Self { status }
    }
}

struct Board {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Board {
    fn new(width: u32, height: u32) -> Self {
        let mut cells: Vec<Cell> = vec![];
        let mut rng = rand::thread_rng();
        for i in 0..(width * height) {
            let rnd: f32 = rng.gen();
            cells.push(Cell::new(if rnd > 0.5 {
                CellStatus::Alive
            } else {
                CellStatus::Dead
            }));
        }
        Self {
            width,
            height,
            cells,
        }
    }

    fn new_empty(width: u32, height: u32) -> Self {
        let mut cells: Vec<Cell> = vec![];
        for i in 0..(width * height) {
            cells.push(Cell::new(CellStatus::Dead));
        }
        Self {
            width,
            height,
            cells,
        }
    }

    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            // let x = (i % WIDTH as usize) as u32;
            // let y = (i / WIDTH as usize) as u32;

            if let Some(cell) = self.cells.get(i) {
                if cell.status == CellStatus::Alive {
                    pixel.copy_from_slice(&[0xff, 0x00, 0x00, 0xff])
                } else {
                    pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff])
                }
            };
        }
    }

    fn get_cell_status(&self, x: i32, y: i32) -> Option<CellStatus> {
        if x < 0 || x >= (self.width as i32) {
            return None;
        };
        if y < 0 || y >= (self.height as i32) {
            return None;
        };
        let i = (y * (self.width as i32) + x) as usize;
        Some(self.cells[i].status)
    }

    fn neighbors_alive_count(&self, idx: u32) -> u32 {
        let x = (idx % self.width) as i32;
        let y = (idx / self.width) as i32;
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if let Some(status) = self.get_cell_status(x + dx, y + dy) {
                    if status == CellStatus::Alive {
                        count += 1
                    }
                };
            }
        }
        count
    }

    fn turn(&self) -> Self {
        let mut cells = Vec::with_capacity(self.cells.len());
        for (idx, cell) in self.cells.iter().enumerate() {
            let nc = self.neighbors_alive_count(idx as u32);
            if cell.status == CellStatus::Alive {
                if nc < 2 || nc > 3 {
                    cells.push(Cell::new(CellStatus::Dead));
                } else {
                    cells.push(Cell::new(CellStatus::Alive));
                }
            } else if nc == 3 {
                cells.push(Cell::new(CellStatus::Alive));
            } else {
                cells.push(Cell::new(CellStatus::Dead));
            }
        }
        Self {
            width: self.width,
            height: self.height,
            cells,
        }
    }
}

fn main() -> Result<(), Error> {
    let mut board = Board::new(WIDTH, HEIGHT);
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Game of Life")
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
        *control_flow = ControlFlow::Poll;
        if let Event::RedrawRequested(_) = event {
            board.draw(pixels.get_frame());
            pixels.render().unwrap()
        }
        if input.update(event) {
            board = board.turn();
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

#[cfg(test)]
mod tests {
    use crate::Board;

    #[test]
    fn neighbour_count() {
        let board = Board::new_empty(3, 3);
        let count = board.neighbors_alive_count(4);
        assert_eq!(count, 0);
        let board = Board::new_empty(3, 2);
        let count = board.neighbors_alive_count(4);
        assert_eq!(count, 0);
    }
}
