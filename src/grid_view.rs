use graphics::types::Color;
use graphics::{Context, Graphics};
use rand::Rng;
use rand::thread_rng;
use crate::GridController;


pub struct GridViewSettings {
    // position from left-top corner
    pub position: [f64; 2],
    // size of grid along horizontal and vertical edge
    pub size: f64,
    pub background_color: Color,
    pub border_color: Color,
    pub border_edge_color: Color,
    pub section_edge_color: Color,
    pub cell_edge_color: Color,
    pub board_edge_radius: f64,
    pub section_edge_radius: f64,
    pub cell_edge_radius: f64,
    pub selected_cell_background_color: Color,
    // number of cells on each axis
    pub num_cells: u32,
}

impl GridViewSettings {
    pub fn new() -> GridViewSettings {
        GridViewSettings {
            position: [39.0, 45.0],
            size: 400.0,
            background_color: [0.2, 0.5, 0.7, 0.7],
            border_color: [0.0, 0.0, 0.2, 1.0],
            border_edge_color: [0.0, 0.0, 0.2, 1.0],
            section_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_edge_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 1.0,
            selected_cell_background_color: [0.9, 0.9, 1.0, 1.0],
            num_cells: 8,
        }
    }
}

pub struct GridView {
    pub settings: GridViewSettings,
}

impl GridView {
    pub fn new(settings: GridViewSettings) -> GridView {
        GridView {
            settings: settings,
        }
    }

    pub fn draw<G: Graphics>(&self,
                             controller: &mut GridController,
                             c: &Context,
                             g: &mut G)
    {
        use graphics::{Line, Rectangle};

        let ref settings = self.settings;
        let grid_rect = [
            settings.position[0], settings.position[1],
            settings.size, settings.size,
        ];

        // draw background
        Rectangle::new(settings.background_color)
            .draw(grid_rect, &c.draw_state, c.transform, g);

        // colour cells
        let cell_size = settings.size / settings.num_cells as f64;
        for i in 0..settings.num_cells {
            let x = settings.position[0] + i as f64 / settings.num_cells as f64 * settings.size;
            for j in 0..settings.num_cells {
                let y = settings.position[1] + j as f64 / settings.num_cells as f64 * settings.size;
                let cell_rect = [x, y, cell_size, cell_size];
                let ref cell_num = i * 8 + j;
                if let Some(n) = controller.grid.cmap.get(cell_num) {
                    //println!("{} {:?}", i, n);
                    Rectangle::new(*n)
                        .draw(cell_rect, &c.draw_state, c.transform, g);
                }
            }
        }
    }
}
