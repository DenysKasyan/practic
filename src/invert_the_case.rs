#[test]
fn test_invert_case() {
    let test_cases = [
        ("Hello", "hELLO"),
        ("lalaLA", "LALAla"),
    ];

    for &(input, expected) in &test_cases {
        assert_eq!(invert_case(input), expected);
        assert_eq!(invert_case(expected), input);
    }
}

fn invert_case(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if c.is_lowercase() {
            result.push(c.to_uppercase().next().unwrap());
        } else if c.is_uppercase() {
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}
