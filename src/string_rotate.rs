#[test]
fn test()
{
    let s = "abcdefgh";
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    shifts
        .iter()
        .for_each(|(n, exp)|
            assert_eq!(
                rotate(s.to_string(), *n),
                exp.to_string()
            )
        );


}
fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    let n = n.rem_euclid(len);
    let split_index = (len - n) as usize;
    let (right, left) = s.split_at(split_index);
    format!("{}{}", left, right)
}