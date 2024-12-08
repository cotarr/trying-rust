#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    // Example, force a panic
    //
    // panic!("crash and burn");

    // Example of fatal execution error, exceed index
    let v = vec![1, 2, 3];
    // Error: index out of bounds: the len is 3 but the index is 99
    v[99];
}
