struct Point<T> {
    x: T,
    y: T
}

struct Line<T, V> {
    start: Point<T>,
    end: Point<V>
}
pub fn generic() {
    let a: Point<i32> = Point { x: 0, y: 0 };  // there is also type inference.... we can also annotate like this!
    let b = Point { x: 1.2, y: 3.4 };
    let myline = Line {start: a, end: b};
}
