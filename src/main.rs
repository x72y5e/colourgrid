extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rand;
extern crate rayon;

use glutin_window::GlutinWindow;
use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::RenderEvent;
use opengl_graphics::{OpenGL, GlGraphics};

use crate::grid::Grid;
use crate::controller::GridController;
use crate::grid_view::{GridView, GridViewSettings};

mod grid;
mod controller;
mod grid_view;


fn main() {
    let opengl = OpenGL::V3_2;
    let dims = [475; 2];
    let settings = WindowSettings::new("colour grid", dims)
        .opengl(opengl)
        .exit_on_esc(true);

    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let mut grid = Grid::new();
    let mut grid_controller = GridController::new(grid);
    let grid_view_settings = GridViewSettings::new();
    let grid_view = GridView::new(grid_view_settings);

    while let Some(e) = events.next(&mut window) {
        grid_controller.event(grid_view.settings.position,
                              grid_view.settings.size,
                              &e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};
                clear([1.0; 4], g);
                grid_view.draw(&mut grid_controller, &c, g)
            });
        }
    }
}
