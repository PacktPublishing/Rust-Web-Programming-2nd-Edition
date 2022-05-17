

struct Coordinate <T> {
    x: T,
    y: T
}


struct SecondCoordinate <T, X> {
    x: T,
    y: X
}


fn main() {
    let one = Coordinate{x: 50, y: 50};
    let two = Coordinate{x: 500, y: 500};
    let three = Coordinate{x: 5.6, y: 5.6};
    let four = Coordinate{x: 50, y: 500};
    let five = Coordinate{x: 5.6, y: 500};
    let six = Coordinate{x: 5.6, y: 50};
}

