fn main() {
    let width: u8 = 8;
    let height: u8 = 2;
    for y in 0..height {
        print!("\n");
        for x in 0..width {
            print!("{},{} ", x, y);
        }
    }
}
