struct Point {
    x: i32,
    y: i32,
}

struct Miles(i32);

fn multiplier(point: Point) -> i32 {
    return point.x * point.y
}

fn main() {
    let point = Point { x: 2, y: 7 };
    let multiplied_number = multiplier(point);

    println!("point.x multiplied={}", multiplied_number);

    let distance = Miles(20);
    let Miles(miles) = distance;
    println!("distance in miles={}", miles);
}
