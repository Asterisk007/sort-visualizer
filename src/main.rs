/*
    Daniel Ellingson
    Sorting algorithm visualizer
    using starting code from https://rust-sdl2.github.io/rust-sdl2/sdl2/index.html
 */

extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;
use rand;
use rand::seq::SliceRandom;
use sdl2::mouse::*;
 
pub fn main() {
    // BEGIN SDL2 SETUP
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let rect_count = 128;
    let max_rect_height = 600;
    let rect_width = 6;

    #[allow(non_snake_case)]
    let SCREEN_WIDTH: u32 = rect_count*rect_width;
    #[allow(non_snake_case)]
    let SCREEN_HEIGHT: u32 = max_rect_height;

    let window = video_subsystem.window("rust-sdl2 demo", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    // END SDL2 SETUP

    // Initialize all rectangle heights
    let mut rect_height: Vec<u32> = (4..=max_rect_height).step_by(4).collect();
    let mut selected: i32 = -1;

    // Shuffle them.
    let mut rng = rand::thread_rng();
    rect_height.shuffle(&mut rng);

    let mut sort = false;

    'running: loop {
        // Have to do this or things get REAL trippy.
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
                    sort = true;
                }
                _ => {}
            }
        }

        // Draw each rectangle based on prior defined variables.
        let mut cur_rect = 0;
        for i in 0..rect_height.clone().len()-1 {
            let height = rect_height[i];
            if i == selected as usize {
                canvas.set_draw_color(Color::RED);
            } else {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
            }
            canvas.fill_rect(
                Rect::new(
                    cur_rect*rect_width as i32,
                    (SCREEN_HEIGHT - height) as i32,
                    rect_width,
                    height
                )
            ).unwrap();
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.draw_rect(
                Rect::new(
                    cur_rect*rect_width as i32,
                    (SCREEN_HEIGHT - height) as i32,
                    rect_width,
                    height
                )
            ).unwrap();
            cur_rect+=1;
        }
        
        // Begin sorting:
        if sort{
            for i in 0..rect_height.clone().len()-1 {
                // Start at first element, compare to next
                if rect_height[i] <= rect_height[i+1] {
                    // Heights are in correct order, continue
                    continue;
                } else {
                    // Heights are not in correct order. Swap them.
                    rect_height.swap(i, i+1);
                    selected = i as i32;
                    break;
                }
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }
}