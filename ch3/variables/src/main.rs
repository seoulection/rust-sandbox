fn main() {
    // tuples with explicit typing
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // types can be inferred
    let tup = (500, 6.4, 1);

    // tuples can be destructured
    let (x, y, z) = tup;

    // tuples values can be accessed
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // arrays have fixed lengths and must be the same type
    let a = [1, 2, 3, 4, 5];

    // arrays with explicit typing and length indicator
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // initialize array of certain length with the same value
    let a = [3; 5]; // [3, 3, 3, 3, 3]

    // accessing using index
    let first = a[0];
    let second = a[1];
}
