/*
    Daniel Ellingson
    Sorting algorithm visualizer
    using starting code from https://rust-sdl2.github.io/rust-sdl2/sdl2/index.html
 */

use raylib::prelude::*;
use std::time::Duration;
use rand;
use rand::seq::SliceRandom;
 
pub fn main() {
    // Window size is depedent of these variables
    let max_rect_height = 600;
    let rect_width = 6;
    let rect_height_step = 4;
    
    // Initialize all rectangle heights
    let mut rect_height: Vec<i32> = (rect_width..=max_rect_height).step_by(rect_height_step as usize).collect();
    let mut selected: i32 = -1;
    
    let SCREEN_HEIGHT: i32 = max_rect_height;
    let SCREEN_WIDTH: i32 = (rect_height.len() as i32 - 1) * rect_width;

    // Shuffle them.
    let mut rng = rand::thread_rng();
    rect_height.shuffle(&mut rng);

    let mut sort = false;

    // Setup raylib window
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Sorting algorithm visualizer")
        .build();

    while !rl.window_should_close() {
        let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        let mut cur_rect = 0;
        let mut fill_color: Color;
        for i in 0..rect_height.clone().len()-1 {
            let height = rect_height[i];
            if i == selected as usize {
                fill_color = Color::RED;
            } else {
                fill_color = Color::WHITE;
            }
            d.draw_rectangle(cur_rect*rect_width, SCREEN_HEIGHT - height, rect_width, height, fill_color);
            d.draw_rectangle_lines(cur_rect*rect_width, SCREEN_HEIGHT - height, rect_width, height, Color::BLACK);
            cur_rect += 1;
        }

        for i in 0..rect_height.clone().len()-1 {
            selected = i as i32;
            if rect_height[i] <= rect_height[i+1] {
                continue;
            } else {
                rect_height.swap(i, i+1);
                break;
            }
        }
    }
}