const LIVE: char = 'O';
const DEAD: char = ' ';

const WIDTH: u8 = 16;
const HEIGHT: u8 = 8;
const LIFE_LIMIT: f64 = 0.93;
const GENERATIONS: u16 = 2;

const LIFE_LIMIT_ARGS_INDEX: usize = 1;
const GENERATIONS_ARGS_INDEX: usize = 2;
const WIDTH_ARGS_INDEX: usize = 3;

enum CellLife {
    Alive,
    Dead,
}


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
            if args.len() > WIDTH_ARGS_INDEX {
                args[WIDTH_ARGS_INDEX].parse::<u8>().unwrap()
            } else {
                WIDTH
            }
        },
        height: {
            if args.len() > WIDTH_ARGS_INDEX+1 {
                args[WIDTH_ARGS_INDEX+1].parse::<u8>().unwrap()
            } else {
                HEIGHT
            }
        },
        life_minimum: {
            if args.len() > LIFE_LIMIT_ARGS_INDEX {
                args[LIFE_LIMIT_ARGS_INDEX].parse::<f64>().unwrap()
            } else {
                LIFE_LIMIT
            }
        },
        generations: {
            if args.len() > GENERATIONS_ARGS_INDEX {
                args[GENERATIONS_ARGS_INDEX].parse::<u16>().unwrap()
            } else {
                GENERATIONS
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

fn generate_map(parms: &Parms) -> Vec<Vec<CellLife>> {
    let mut map: Vec<Vec<CellLife>> = Vec::new();

    for _y in 0..parms.height {
        let mut row: Vec<CellLife> = Vec::new();
        for _x in 0..parms.width {
            let life = rand::random::<f64>() > parms.life_minimum;
            let life = match life {
                true => CellLife::Alive,
                _ => CellLife::Dead,
            };
            row.push(life);
        }
        map.push(row);
    }
    map
}

fn display_map(map: &Vec<Vec<CellLife>>, gen: &u16) {
    print!("generation: {}\n", gen);
    for y in map {
        for x in y {
            print!("{}", {
                match *x {
                    CellLife::Alive => LIVE,
                    _ => DEAD,
                }
                // if *x == true {
                //     LIVE
                // } else {
                //     DEAD
                // }
            });
        }
        print!("\n");
    }
}

fn calc_next_gen_map(map: &Vec<Vec<CellLife>>) -> Vec<Vec<CellLife>> {
    let mut newmap: Vec<Vec<CellLife>> = Vec::new();

    for (y, row) in map.iter().enumerate() {
        let mut newrow: Vec<CellLife> = Vec::new();
        for (x, _cell) in row.iter().enumerate() {
            let neighbour_count = count_cell_neighbours(map, &x, &y);
            if map[y][x] == CellLife::Alive {
                match neighbour_count {
                    0 | 1 => newrow.push(CellLife::Dead),
                    2 | 3 => newrow.push(CellLife::Alive),
                    _ => newrow.push(CellLife::Dead),
                }
            } else {
                match neighbour_count {
                    3 | 4 => newrow.push(CellLife::Alive),
                    _ => newrow.push(CellLife::Dead),
                }
            }
        }
        newmap.push(newrow);
    }

    newmap
}

fn count_cell_neighbours(map: &Vec<Vec<CellLife>>, x: &usize, y: &usize) -> u16 {
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

            if map[ny][nx] == CellLife::Alive {
                count += 1;
            }
        }
    }
    count
}
