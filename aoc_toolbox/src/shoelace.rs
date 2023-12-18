use crate::point::Point;

pub fn shoelace(points: &[Point]) -> isize {
    let mut sum = 0;
    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];

        sum += (p1.x * p2.y) - (p2.x * p1.y);
    }

    let p1 = points[points.len() - 1];
    let p2 = points[0];
    sum += (p1.x * p2.y) - (p2.x * p1.y);

    if sum % 2 == 1 {
        panic!("shoelace resulted in odd number")
    }
    (sum / 2).abs()
}
