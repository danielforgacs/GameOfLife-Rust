const LIVE: char = 'x';
const DEAD: char = '-';

struct Parms {
    width: u8,
    height: u8,
    life_minimum: f64,
    generations: u16,
}

fn main() {
    let parms = Parms {
        width: 8,
        height: 5,
        life_minimum: 0.8,
        generations: 5,
    };

    let map = generate_map(&parms);
    let mut map: Vec<Vec<bool>> = vec![
        vec![false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false],
        vec![false, true,  true, false, false, false, false, false],
        vec![false, false, true, true, false, false, false, false],
        vec![false, false, true, false, false, false, false, false],
    ];
    display_map(&map, &0_u16);
    // let map = calc_next_gen_map(&map);
    // display_map(&map, &0);
    // let map = calc_next_gen_map(&map);
    // display_map(&map, &0);
    // let mut map = calc_next_gen_map(&map);
    // display_map(&map, &0);

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
            print!("{}", {if *x == true {LIVE} else {DEAD}});
        }
        print!("\n");
    }
}

fn calc_next_gen_map(map: &Vec<Vec<bool>>) -> Vec<Vec<bool>>{
    let mut newmap: Vec<Vec<bool>> = Vec::new();

    for (y, row) in map.iter().enumerate() {
        let mut newrow: Vec<bool> = Vec::new();
        for (x, cell) in row.iter().enumerate(){
            let neighbour_count = count_cell_neighbours(map, &x, &y);
            print!("{}", neighbour_count);
            if map[y][x] == true {
                match neighbour_count {
                    0 | 1 => newrow.push(false),
                    2 | 3 => newrow.push(true),
                    _ => newrow.push(false),
                }
            } else {
                match neighbour_count {
                    3 => newrow.push(true),
                    _ => newrow.push(false),
                }
            }
        }
        newmap.push(newrow);
        print!("\n");
    }

    newmap
}

fn count_cell_neighbours(map: &Vec<Vec<bool>>, x: &usize, y: &usize) -> u8 {
    let mut count = 0;
    let coords: Vec<i8> = vec![-1, 0, 1];

    for offset_x in &coords {
        for offset_y in &coords {
            let nx = *x as i8 + *offset_x;
            let ny = *y as i8 + *offset_y;
            
            if nx < 0 || ny < 0 || ny > { map.len() - 1} as i8 || nx > { map[0].len() -1 } as i8 {
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