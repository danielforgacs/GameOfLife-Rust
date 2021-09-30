extern crate rand;
extern crate termion;
extern crate image;
use std::{env, thread, time};
use std::fs::File;
use std::io::{BufRead, BufReader};
use termion::color;
use termion::clear;

fn census(_world: [[u8; 75]; 75]) -> u16 {
    let mut count = 0;
    for i in 0..75 {
        for j in 0..75 {
            if _world[i][j] == 1 {
                count += 1;
            }
        }
    }
    count
}

fn generation(_world: [[u8; 75]; 75]) -> [[u8; 75]; 75] {
    let mut newworld = [[0_u8; 75]; 75];

    for i in 0..75 {
        for j in 0..75 {
            let mut count = 0;

            if i > 0 {
                count = count + _world[i-1][j];
            }
            if i > 0 && j > 0 {
                count = count + _world[i-1][j-1];
            }
            if i > 0 && j < 74 {
                count = count + _world[i-1][j+1];
            }
            if i < 74 && j > 0 {
                count = count + _world[i+1][j-1];
            }
            if i < 74 {
                count = count + _world[i+1][j];
            }

            if i < 74 && j < 74 {
                count = count + _world[i+1][j+1]
            }
            if  j > 0 {
                count = count + _world[i][j-1]
            }
            if  j < 74 {
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
    let mut world = [[0_u8; 75]; 75];
    let mut generations = 0;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        for i in 0..75 {
            for j in 0..75 {
                if rand::random() {
                    world[i][j] = 1;
                } else {
                    world[i][j] = 0;
                }
            }
        }
        
    } else {

        let filename = env::args().nth(1).unwrap();
        world = populate_from_file(filename);
        // display_world(world);
        println!("ensus{}, gen {}", census(world), generations);
    }
    println!("Population at generation {} is {}", generations, census(world));
    let filename = format!("images/game_of_life.{:04}.png", generations);
    write_image(world, filename);
    // println!("{}", filename);
    // return;
    display_world(world);

    for _gens in 0..25 {
        let temp = generation(world);
        world = temp;
        generations += 1;
        let filename = format!("images/game_of_life.{:04}.png", generations);
        write_image(world, filename);

        // println!("{}", filename);
    
        // println!("{}", clear::All);
        display_world(world);
        println!("{blue}Population at generation {g} is {c}",
            blue=color::Fg(color::Blue),
            g=generations,
            c=census(world));
        // thread::sleep(time::Duration::from_secs_f32(0.1));
    }
}

fn display_world(world: [[u8; 75]; 75]) {
    for i in 0..75 {
        for j in 0..75 {
            if world[i][j] == 1 {
                print!("{red}*", red=color::Fg(color::Red));
            }
            else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn populate_from_file(filename: String) -> [[u8; 75]; 75] {
    let mut newworld = [[0_u8; 75]; 75];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        let mut words = l.split_whitespace();
        let left = words.next().unwrap();
        let rigth = words.next().unwrap();
        pairs.push((left.parse::<usize>().unwrap(), rigth.parse::<usize>().unwrap()))
    }
    for i in 0..75 {
        for j in 0..75 {
            newworld[i][j] = 0;
        }
    }

    for (x,y) in pairs {
        println!("live cell: {}, {}", x, y);
        newworld[x][y] = 1;
    }
    // println!("census {}", census(newworld));
    // display_world(newworld);

    newworld
}

fn write_image(pixels: [[u8; 75]; 75], filename: String) {
    let w = 75_u32;
    let h = 75_u32;
    let mut imgbuf = image::ImageBuffer::new(w, h);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = pixels[x as usize][y as usize];
        let r = r * 255;
        let g = r;
        let b = r;
        *pixel = image::Rgb([r, g, b]);
    }

    // imgbuf.save(filename).unwrap();
    match imgbuf.save(filename) {
        Ok(_) => {},
        // Err(error) => { println!("Could not save for some reason.") }
        Err(error) => { panic!("[ERROR] Could not save for some reason. Check if the \"images\" directory exists!") }
    };
}