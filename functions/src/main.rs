#[allow(unused_variables)]
fn main() {
    another_function(5);

    //parameters must have type annotations
    fn another_function(x: i32) {
        println!("The value of x is {x}");
    }

    print_labeled_measurement(5, 'h');

    fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is {value}{unit_label}");
    }

    //This is a statement. Function definitions are also a statement
    let x = 1;

    /*
    Expressions return a value. Calling a function/macro are also expressions
    In general, blocks of code evaluate to an expression
    */
    let y = {
        let x = 3;
        x + 1 //returns x+1
    };
    println!("The value of y is: {y}");

    /*
    This is wrong because there is a semicolon, changing x + 1 to a statement
    Therefore the function will implicity return the unit type () which doesn't match i32
    fn add_one() -> i32 {
        x + 1;
    }
    */

    let x = five();
    println!("The value of x is {x}");

    fn five() -> i32 {
        5
    }
}
