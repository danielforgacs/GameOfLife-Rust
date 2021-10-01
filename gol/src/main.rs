const LIVE: char = 'O';
const DEAD: char = ' ';

const WIDTH: u16 = 16;
const HEIGHT: u16 = 8;
const LIFE_LIMIT: f64 = 0.7;
const GENERATIONS: u16 = 2;

const LIFE_LIMIT_ARGS_INDEX: usize = 1;
const GENERATIONS_ARGS_INDEX: usize = 2;
const WIDTH_ARGS_INDEX: usize = 3;

struct Cell {
    life: CellLife,
    x: u16,
    y: u16,
}

struct World {
    width: u16,
    height: u16,
    generation: u16,
    cells: Vec<Vec<Cell>>,
}

impl World {
    fn new(parms: &Parms) -> Self {
        let mut world = World {
            width: parms.width,
            height: parms.height,
            generation: 0,
            cells: Vec::new(),
        };

        for y in 0..world.height {
            let mut row: Vec<Cell> = Vec::new();
            for x in 0..world.width {
                let life = rand::random::<f64>() > parms.life_minimum;
                let life = match life {
                    true => CellLife::Alive,
                    _ => CellLife::Dead,
                };
    
                let cell = Cell {
                    life: life,
                    x: x,
                    y: y,
                };
                // print!("({},{}:{}) ", &cell.x, &cell.y, { match cell.life { CellLife::Alive => LIVE, _ => DEAD } });
                row.push(cell);
            }
            // print!("\n");
            world.cells.push(row);

        }

        world
    }

    fn alive_count(&self) -> u64 {
        let mut alive_count = 0_u64;
        for row in &self.cells {
            for cell in row {
                match cell.life {
                    CellLife::Alive => alive_count += 1,
                    _ => {},
                }
            }
        };
        alive_count
    }

    fn display(&self) {
        for row in &self.cells {
            for cell in row {
                print!("{}", match cell.life{ CellLife::Alive => { LIVE }, _ => { DEAD }});
            } 
            print!("\n");
        }
    }

    fn next_generation(&mut self) {
        let newcells = calc_next_gen_map(&self.cells);
        self.cells = newcells;
        self.generation += 1;
    }
}

enum CellLife {
    Alive,
    Dead,
}

struct Parms {
    width: u16,
    height: u16,
    life_minimum: f64,
    generations: u16,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let parms = Parms {
        width: {
            if args.len() > WIDTH_ARGS_INDEX {
                args[WIDTH_ARGS_INDEX].parse::<u16>().unwrap()
            } else {
                WIDTH
            }
        },
        height: {
            if args.len() > WIDTH_ARGS_INDEX + 1 {
                args[WIDTH_ARGS_INDEX + 1].parse::<u16>().unwrap()
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

    let mut world = World::new(&parms);
    print!(
        "-------------------------\n\
        generation: {}\n\
        alive count: {}\n",
        world.generation, world.alive_count()
    );
    world.display();

    for _k in 0..parms.generations {

            world.next_generation();
            print!(
                "-------------------------\n\
                generation: {}\n\
                alive count: {}\n",
                world.generation, world.alive_count()
            );
            world.display();
    }
    // return;

    // let mut map = generate_map(&parms);
    // display_map(&map, &0_u16);

    // for gen in 1..parms.generations {
    //     map = calc_next_gen_map(&map);
    //     display_map(&map, &gen);
    // }

    print!(
        "==============================\n\
        dimexnsions: {dimx}x{dimy}\n\
        life treshhold: {lif}\n\
        generations: {gen}\n\
        ",
        lif=parms.life_minimum,
        dimx=parms.width,
        dimy=parms.height,
        gen=parms.generations,
    );
}

fn generate_map(parms: &Parms) -> Vec<Vec<Cell>> {
    let mut map: Vec<Vec<Cell>> = Vec::new();

    for y in 0..parms.height {
        let mut row: Vec<Cell> = Vec::new();

        for x in 0..parms.width {
            let life = rand::random::<f64>() > parms.life_minimum;
            let life = match life {
                true => CellLife::Alive,
                _ => CellLife::Dead,
            };
            row.push(Cell {
                life: life,
                x: x,
                y: y,
            });
        }
        map.push(row);
    }
    map
}

fn display_map(map: &Vec<Vec<Cell>>, gen: &u16) {
    for y in map {
        for x in y {
            print!("{}", {
                match x.life {
                    CellLife::Alive => LIVE,
                    _ => DEAD,
                }
            });
        }
        print!("\n");
    }
}

fn calc_next_gen_map(map: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut newmap: Vec<Vec<Cell>> = Vec::new();

    for (y, row) in map.iter().enumerate() {
        let mut newrow: Vec<Cell> = Vec::new();

        for (x, _cell) in row.iter().enumerate() {
            let neighbour_count = count_cell_neighbours(map, &x, &y);
            let life = match map[y][x].life {
                CellLife::Alive => match neighbour_count {
                    0 | 1 => CellLife::Dead,
                    2 | 3 => CellLife::Alive,
                    _ => CellLife::Dead,
                },
                _ => match neighbour_count {
                    3 => CellLife::Alive,
                    4 => CellLife::Dead,
                    _ => CellLife::Dead,
                },
            };

            let newcell = Cell {
                life: life,
                x: x as u16,
                y: y as u16,
            };

            newrow.push(newcell);
        }

        newmap.push(newrow);
    }

    newmap
}

fn count_cell_neighbours(map: &Vec<Vec<Cell>>, x: &usize, y: &usize) -> u16 {
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

            match map[ny][nx].life {
                CellLife::Alive => {
                    count += 1;
                }
                _ => {}
            }
        }
    }
    count
}
