use linal::{matrix::Matrix};

fn main() {
    let m = Matrix::<3, 3, f32>::from([
        [8.0, 3.0, 2.0],
        [1.0, 7.0, 9.0],
        [5.0, 3.0, 3.0],
    ]);

    println!("{}", m.det());
}