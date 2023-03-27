/* Copyright 2021 mrcl dbbrk
 * SPDX-License-Identifier: Apache-2.0
 */

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::video::{WindowSurfaceRef, FullscreenType};
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::collections::{HashSet, HashMap};

fn update_living_cells(living_cells: &mut HashSet<(i32, i32)>)
{
    fn as_living(cell: (i32, i32),
                  candidates: &mut HashMap<(i32, i32), (u32, bool)>)
    {
        match candidates.get_mut(&cell) {
            Some((_, lives)) => *lives = true,
            _ => {candidates.insert(cell, (0, true));}
        }
    }

    fn as_neighbor_to_living(cell: (i32, i32),
                             candidates: &mut HashMap<(i32, i32), (u32, bool)>)
    {
        match candidates.get_mut(&cell) {
            Some((living_neighbors, _)) => *living_neighbors += 1,
            _ => {candidates.insert(cell, (1, false));}
        }
    }

    let mut candidates = HashMap::new();

    for &(x, y) in living_cells.iter() {
        as_living((x, y), &mut candidates);
        as_neighbor_to_living((x-1, y-1), &mut candidates);
        as_neighbor_to_living((  x, y-1), &mut candidates);
        as_neighbor_to_living((x+1, y-1), &mut candidates);
        as_neighbor_to_living((x-1,   y), &mut candidates);
        as_neighbor_to_living((x+1,   y), &mut candidates);
        as_neighbor_to_living((x-1, y+1), &mut candidates);
        as_neighbor_to_living((  x, y+1), &mut candidates);
        as_neighbor_to_living((x+1, y+1), &mut candidates);
    }

    living_cells.clear();

    for (c, (n, l)) in candidates {
        if n == 3 || (n == 2 && l) {
            living_cells.insert(c);
        }
    }
}

struct State {
    fill_color: Color,
    draw_color: Color,
    pause: bool,
    scale: i32,
    translate: (i32, i32),
    living_cells: HashSet<(i32, i32)>
}

impl State {
    fn insert_cell(&mut self, x: i32, y: i32) {
        self.living_cells.insert(self.screen_to_world((x, y)));
    }

    fn remove_cell(&mut self, x: i32, y: i32) {
        self.living_cells.remove(& self.screen_to_world((x, y)));
    }

    fn change_zoom_by(&mut self, z: i32) {
        self.scale += z;
        if self.scale < 1 { self.scale = 1 }
        else if self.scale > 8 { self.scale = 8 }
    }

    fn screen_to_world(&self, (x, y): (i32, i32)) -> (i32, i32) {
        ((x - self.translate.0) / self.scale, (y - self.translate.1) / self.scale)
    }

    fn world_to_screen(&self, (x, y): (i32, i32)) -> (i32, i32) {
        ((x * self.scale + self.translate.0), (y * self.scale + self.translate.1))
    }

    fn change_zoom_to(&mut self, z: i32) {
        self.scale = z;
        if self.scale < 1 { self.scale = 1 }
        else if self.scale > 8 { self.scale = 8 }
    }

    fn update(&mut self) {
        if self.pause {return;}
        update_living_cells(&mut self.living_cells);
    }

    fn draw(&self, window_surface: &mut WindowSurfaceRef) {
        window_surface.fill_rect(None, self.fill_color).unwrap();
        for c in &self.living_cells {
            let (x, y) = self.world_to_screen(*c);
            window_surface.fill_rect(
                Some(Rect::new(x, y, self.scale as u32, self.scale as u32)),
                self.draw_color).unwrap();
        }
    }
}

fn main() {
    let state = &mut State {
        fill_color: Color::BLUE,
        draw_color: Color::RED,
        pause: true,
        scale: 5,
        translate: (400, 300),
        living_cells: HashSet::new()
    };

    let init = sdl2::init().unwrap();

    let video = init
        .video()
        .unwrap();

    let mut event_pump = init
        .event_pump()
        .unwrap();

    let mut timer = init
        .timer()
        .unwrap();

    let mut window = video
        .window("sdl", 800, 600)
        .resizable()
        .build()
        .unwrap();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..}
                 => break 'main,
                Event::KeyDown {keycode: Some(key), ..}
                 => match key {
                        Keycode::Space
                         => state.pause = !state.pause,
                        Keycode::C
                         => state.living_cells = HashSet::new(),
                        Keycode::F
                         => { window.set_fullscreen
                                ( if window.fullscreen_state()
                                      == FullscreenType::Off
                                    { FullscreenType::Desktop }
                                  else
                                    { FullscreenType::Off } ).unwrap(); },
                        _
                         => {}
                    },
                Event::MouseButtonDown {x, y, mouse_btn: MouseButton::Left, ..}
                  => state.insert_cell(x, y),
                Event::MouseMotion {x, y, mousestate, ..} if mousestate.left()
                  => state.insert_cell(x, y),
                Event::MouseButtonDown {x, y, mouse_btn: MouseButton::Right, ..}
                  => state.remove_cell(x, y),
                Event::MouseMotion {x, y, mousestate, ..} if mousestate.right()
                  => state.remove_cell(x, y),
                Event::MouseMotion {xrel, yrel, mousestate, ..}
                if mousestate.middle()
                  => {state.translate.0 += xrel; state.translate.1 += yrel;},
                Event::MouseWheel {y, ..} if y > 0
                  => state.change_zoom_by( 1),
                Event::MouseWheel {y, ..} if y < 0
                  => state.change_zoom_by(-1),
                _
                  => {}
            }
        }

        let mut window_surface = window.surface(&event_pump).unwrap();

        state.update();
        state.draw(&mut window_surface);

        timer.delay(30);

        window_surface.update_window().unwrap();
    }
}
