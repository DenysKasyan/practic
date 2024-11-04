#[test]
fn test_invert_case() {
    let data = [
        ("Hello", "hELLO"),
        ("dadaDA", "DAdADa"),
    ];

    data
        .iter()
        .for_each(|(a, b)| {
            assert_eq!(
                invert_case(a.to_string()),
                b.to_string()
            );
            assert_eq!(
                invert_case(b.to_string()),
                a.to_string()
            );
        });

}

fn invert_case(s: String) -> String {
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
