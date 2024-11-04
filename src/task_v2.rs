fn find_values() -> Vec<(u16, u16, u16, u16, u16, u16, u16, u16)> {
    let mut solutions = Vec::new();

    for m in 1..=8 {
        for u in 1..=8 {
            if m == u { continue; }
            for x in 1..=8 {
                if m == x || u == x { continue; }
                for a in 1..=8 {
                    if m == a || u == a || x == a { continue; }
                    for s in 1..=8 {
                        if m == s || u == s || x == s || a == s { continue; }
                        for l in 1..=8 {
                            if m == l || u == l || x == l || a == l || s == l { continue; }
                            for o in 1..=8 {
                                if m == o || u == o || x == o || a == o || s == o || l == o { continue; }
                                for n in 1..=8 {
                                    if m == n || u == n || x == n || a == n || s == n || l == n || o == n { continue; }
                                    let muxa = m * 1000 + u * 100 + x * 10 + a;
                                    let slon = s * 1000 + l * 100 + o * 10 + n;
                                    if muxa * x == slon {
                                        solutions.push((m, u, x, a, s, l, o, n));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    solutions
}

#[test]
fn main() {
    let solutions = find_values();

    for (m, u, x, a, s, l, o, n) in solutions.iter() {
        println!("{}{}{}{}", m, u, x, a);
        println!("x        {}", x);
        println!("  ------");
        println!("    {}{}{}{}", s, l, o, n);
    }

    println!("Кількість рішень: {}", solutions.len());
}
