/*
    Rust is a statically typed language:
    all types need to be known at compile time
*/

#[allow(unused_variables)]
fn main() {
    /*
        Integer types:
        Length    Signed    Unsigned
        8-bit     i8        u8
        16-bit    i16       u16
        32-bit    i32       u32
        64-bit    i64       u64
        128-bit   i128      u128
        arch.     isize     usize
    */

    //stores numbers from -(2^31) to 2^31 - 1 inclusive
    let x: i32 = 1;
    let a_million = 1_000_000;
    let hex = 0xfff;
    println!("{x}, {a_million}, {hex}");

    //f64 (double-precision) is default
    let x = 2.0;
    let y: f32 = 3.0;
    println!("{x}, {y}");

    let sum = 1 + 1;
    let difference = 3 - 1;
    let product = 2 * 5;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; //truncated to -1
    let remainder = 5 % 2;
    println!("{sum}, {difference}, {product}, {quotient}, {truncated}, {remainder}");

    //can only be true/false
    let is_happy = true;
    let is_sad: bool = false;
    println!("{is_happy}, {is_sad}");

    let c = 'c';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{c}, {z}, {heart_eyed_cat}");

    //primitive compound types: tuples and arrays

    let tup: (i32, f64, u8) = (500, 6.4, 7);
    let (x, y, z) = tup; //destructuring
    let x2 = tup.0;
    let y2 = tup.1;
    let _unit_tuple = (); //expressions implicity return this
    println!("{x}, {y}, {z}, {x2}, {y2}");

    //arrays are immutable and are stored on the stack
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [3; 5]; //let _b = [3, 3, 3, 3, 3];
                               //programs panics if index is out of bounds
    let first = a[0];
    let second = a[2];
    println!("{first}, {second}");
}
