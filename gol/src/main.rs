const LIVE: char = 'x';
const DEAD: char = '-';

struct Parms {
    width: u8,
    height: u8,
    life_minimum: f64,
    generations: u16,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let parms = Parms {
        width: {
            if args.len() > 1 {
                args[1].parse::<u8>().unwrap()
            } else {
                64
            }
        },
        height: {
            if args.len() > 2 {
                args[2].parse::<u8>().unwrap()
            } else {
                32
            }
        },
        life_minimum: {
            if args.len() > 3 {
                args[3].parse::<f64>().unwrap()
            } else {
                0.93
            }
        },
        generations: {
            if args.len() > 4 {
                args[4].parse::<u16>().unwrap()
            } else {
                2
            }
        },
    };
    // println!("{}", parms.width);
    // return;

    let mut map = generate_map(&parms);
    display_map(&map, &0_u16);

    for gen in 1..parms.generations {
        map = calc_next_gen_map(&map);
        display_map(&map, &gen);
    }
}

fn generate_map(parms: &Parms) -> Vec<Vec<bool>> {
    let mut map: Vec<Vec<bool>> = Vec::new();

    for _y in 0..parms.height {
        let mut row: Vec<bool> = Vec::new();
        for _x in 0..parms.width {
            let life = rand::random::<f64>() > parms.life_minimum;
            row.push(life);
        }
        map.push(row);
    }
    map
}

fn display_map(map: &Vec<Vec<bool>>, gen: &u16) {
    print!("generation: {}\n", gen);
    for y in map {
        for x in y {
            print!("{}", {
                if *x == true {
                    LIVE
                } else {
                    DEAD
                }
            });
        }
        print!("\n");
    }
}

fn calc_next_gen_map(map: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut newmap: Vec<Vec<bool>> = Vec::new();

    for (y, row) in map.iter().enumerate() {
        let mut newrow: Vec<bool> = Vec::new();
        for (x, _cell) in row.iter().enumerate() {
            let neighbour_count = count_cell_neighbours(map, &x, &y);
            if map[y][x] == true {
                match neighbour_count {
                    0 | 1 => newrow.push(false),
                    2 | 3 => newrow.push(true),
                    _ => newrow.push(false),
                }
            } else {
                match neighbour_count {
                    3 | 4 => newrow.push(true),
                    _ => newrow.push(false),
                }
            }
        }
        newmap.push(newrow);
    }

    newmap
}

fn count_cell_neighbours(map: &Vec<Vec<bool>>, x: &usize, y: &usize) -> u16 {
    let mut count = 0;
    let coords: Vec<i16> = vec![-1, 0, 1];

    for offset_x in &coords {
        for offset_y in &coords {
            let nx = *x as i16 + *offset_x;
            let ny = *y as i16 + *offset_y;

            if nx < 0 || ny < 0 || ny > { map.len() - 1 } as i16 || nx > { map[0].len() - 1 } as i16
            {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            if (nx, ny) == (*x, *y) {
                continue;
            }

            if map[ny][nx] == true {
                count += 1;
            }
        }
    }
    count
}
