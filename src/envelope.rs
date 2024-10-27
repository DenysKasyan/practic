#[test]
fn test() {
    const W: u32 = 25;
    const H: u32 = 10;
    for y in 0..H {
        for x in 0..W {
            let is_horizontal = y == 0 || y == H - 1;
            let is_vertical = x == 0 || x == W - 1;
            let is_diagonal1 = x == y * (W - 1) / (H - 1);
            let is_diagonal2 = x == (H - y - 1) * (W - 1) / (H - 1);
            let c = if is_horizontal || is_vertical || is_diagonal1 || is_diagonal2 {
                '*'
            } else {
                ' '
            };
            print!("{}", c);
        }
        println!();
    }
}
