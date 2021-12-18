extern crate rand;
extern crate termion;
extern crate image;
use std::{env, thread, time};
use std::fs::File;
use std::io::{BufRead, BufReader};
use termion::color;
use termion::clear;

const WIDTH: usize = 100;
const HEIGHT: usize = 75;
const GENERATIONS: i32 = 5;

fn census(_world: [[u8; WIDTH]; HEIGHT]) -> u16 {
    let mut count = 0;
    for i in 0..HEIGHT-1{
        for j in 0..WIDTH-1 {
            if _world[i][j] == 1 {
                count += 1;
            }
        }
    }
    count
}

fn generation(_world: [[u8; WIDTH]; HEIGHT]) -> [[u8; WIDTH]; HEIGHT] {
    let mut newworld = [[0_u8; WIDTH]; HEIGHT];

    for i in 0..HEIGHT-1{
        for j in 0..WIDTH-1{
            let mut count = 0;

            if i > 0 {
                count = count + _world[i-1][j];
            }
            if i > 0 && j > 0 {
                count = count + _world[i-1][j-1];
            }
            if i > 0 && j < WIDTH-1 {
                count = count + _world[i-1][j+1];
            }
            if i < HEIGHT-1 && j > 0 {
                count = count + _world[i+1][j-1];
            }
            if i < HEIGHT-1{
                count = count + _world[i+1][j];
            }

            if i < HEIGHT-1 && j < WIDTH-1{
                count = count + _world[i+1][j+1]
            }
            if  j > 0 {
                count = count + _world[i][j-1]
            }
            if  j < WIDTH-1{
                count = count + _world[i][j+1]
            }

            newworld[i][j] = 0;

            if (count < 2) && (_world[i][j] == 1) {
                newworld[i][j] = 0;
            }
            if (_world[i][j] == 1) && (count == 2 || count == 3) {
                newworld[i][j] = 1;
            }
            if (_world[i][j] == 0) && (count == 3) {
                newworld[i][j] = 1;
            }
        }
    }

    newworld
}

fn main() {
    let mut world = [[0_u8; WIDTH]; HEIGHT];
    let mut generations = 0;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        for i in 0..HEIGHT-1{
            for j in 0..WIDTH-1 {
                if rand::random() {
                    world[i][j] = 1;
                } else {
                    world[i][j] = 0;
                }
            }
        }
    }
    println!("Population at generation {} is {}", generations, census(world));
    let filename = format!("images/game_of_life.{:04}.png", generations);
    write_image(world, filename);

    for _gens in 0..GENERATIONS {
        let temp = generation(world);
        world = temp;
        generations += 1;
        let filename = format!("images/game_of_life.{:04}.png", generations);
        write_image(world, filename);
        println!("{blue}Population at generation {g} is {c}",
            blue=color::Fg(color::Blue),
            g=generations,
            c=census(world));
    }
}

fn write_image(pixels: [[u8; WIDTH]; HEIGHT], filename: String) {
    let w = WIDTH as u32;
    let h = HEIGHT as u32;
    let mut imgbuf = image::ImageBuffer::new(w, h);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = pixels[y as usize][x as usize];
        let r = r * 255;
        let g = r;
        let b = r;
        *pixel = image::Rgb([r, g, b]);
    }

    match imgbuf.save(filename) {
        Ok(_) => {},
        Err(error) => { panic!("[ERROR] Could not save for some reason. Check if the \"images\" directory exists!") }
    };
}
