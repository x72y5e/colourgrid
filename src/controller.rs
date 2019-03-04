use piston::input::GenericEvent;
use crate::rand::Rng;
use crate::rand::thread_rng;
use crate::grid::Grid;


pub struct GridController {
    pub grid: Grid,
    pub selected_cell: Option<[usize; 2]>,
    cursor_pos: [f64; 2],
}

impl GridController {
    pub fn new(grid: Grid) -> GridController {
        GridController {
            grid: grid,
            selected_cell: None,
            cursor_pos: [0.0; 2],
        }
    }

    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E) {

        use piston::input::{Button, MouseButton};

        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            // find coords relative to upper left
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];
            // check inside boundaries
            if x >= 0.0 && x < size && y >= 0.0 && y < size {
                let mut rng = thread_rng();
                let cell_x = (x / size * self.grid.size as f64) as usize;
                let cell_y = (y / size * self.grid.size as f64) as usize;
                self.selected_cell = Some([cell_x, cell_y]);
                let colour: [f32; 4] = [rng.gen(), rng.gen(), rng.gen(), rng.gen()];
                let cell_num = cell_x * 8 + cell_y;
                self.grid.cmap.insert(cell_num as u32, colour);
            }
        }
    }
}
