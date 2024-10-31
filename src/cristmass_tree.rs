#[test]
fn test1() {
    let pov = 5;
    for t in 0..pov
    {
        for i in 0..=t
        {
            for _ in 0..pov - i
            {
                print!(" ");
            }
            for _ in 0..=i * 2
            {
                print!("*");
            }
            println!();
        }
    }
}

