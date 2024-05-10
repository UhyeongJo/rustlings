// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}

// hint
// The compiler says a partial move happened in the `match` statement. How can
// this be avoided? The compiler shows the correction needed.
//
// After making the correction as suggested by the compiler, do read:
// https://doc.rust-lang.org/std/keyword.ref.html