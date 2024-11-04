use std::cmp::{max, min};

struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
}

fn intersection_area(r1: &Rectangle, r2: &Rectangle) -> i32 {
    let x_overlap = max(0, min(r1.b.x, r2.b.x) - max(r1.a.x, r2.a.x));
    let y_overlap = max(0, min(r1.a.y, r2.a.y) - max(r1.b.y, r2.b.y));
    x_overlap * y_overlap
}
fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut total_area = 0;

    for rect in xs {
        total_area += (rect.b.x - rect.a.x).abs() * (rect.a.y - rect.b.y).abs();
    }

    let mut intersection_area_sum = 0;
    for i in 0..xs.len() {
        for j in i + 1..xs.len() {
            intersection_area_sum += intersection_area(&xs[i], &xs[j]);
        }
    }

    total_area - intersection_area_sum
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}
#[test]
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}