struct Coordinate <T, X> {
    x: T,
    y: X
}

fn main() {
    let one = Coordinate{x: 50, y: 500};
    let two = Coordinate{x: 5.6, y: 500};
    let three = Coordinate{x: 5.6, y: 50};
}
