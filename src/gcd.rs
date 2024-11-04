#[test]
fn test1() {
    let test_cases = [
        ((24,  60), 12),
        ((15,   9),  3),
        ((15,   6),  3),
        ((140, 40), 20),
        ((24,  16),  8),
        ((100, 10), 10),
        ((120, 80), 40),
        ((80, 120), 40),
        ((100, 20), 20),
        ((37,  11),  1),
        ((120, 90), 30),
    ];

    for &((a, b), expected) in &test_cases {
        assert_eq!(expected, euclidean(a, b));
    }
    println!("Success");
}

fn euclidean(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
