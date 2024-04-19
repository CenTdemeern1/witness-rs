extern crate sdl2;
#[macro_use]
extern crate witness_core;

use gameplay::grid::{Grid, VertexID};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::gfx::primitives::DrawRenderer;
use std::time::Duration;
use witness_core::*;

const FRAME_RATE: Duration = Duration::new(0, 1_000_000_000u32 / 60);

pub fn main() {
    let grid = Grid::new(vec2!(2, 4));

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let frame_start_time = std::time::Instant::now();

        // Clear the screen and make it black
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                // Quit when escape is pressed or the application is closed
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                // For all other events, do nothing
                _ => {}
            }
        }

        // - Draw grid -

        let vertex_to_drawable_point =
            |v: VertexID| vec2cast!(grid.get_vertex_position(v).unwrap() * 3 + 1, i32).into_tuple();

        // Draw cells
        grid.get_all_cellrefs_iter().for_each(|cell| {
            let (xvec, yvec) = cell.read().unwrap().get_vertices_in_winding_order().into_iter().fold((vec![], vec![]), |mut acc: (Vec<i16>, Vec<i16>), vertex| {
                let point = vertex_to_drawable_point(vertex);
                acc.0.push(point.0 as i16);
                acc.1.push(point.1 as i16);
                acc
            });
            canvas.filled_polygon(&xvec, &yvec, Color::RED).unwrap();
        });

        // Draw edges
        canvas.set_draw_color(Color::BLUE);
        grid.get_all_edgerefs_iter().for_each(|edge| {
            let vertices = edge.read().unwrap().get_vertices_tuple();
            canvas
                .draw_line(
                    vertex_to_drawable_point(vertices.0),
                    vertex_to_drawable_point(vertices.1),
                )
                .unwrap()
        });

        // Draw vertices
        canvas.set_draw_color(Color::GREEN);
        canvas
            .draw_points(
                grid.get_all_vertex_ids_iter()
                    .map(|vertex| vertex_to_drawable_point(vertex).into())
                    .collect::<Vec<Point>>()
                    .as_slice(),
            )
            .unwrap();

        canvas.present();
        std::thread::sleep(FRAME_RATE.saturating_sub(frame_start_time.elapsed()));
    }
}
