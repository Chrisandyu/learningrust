fn main() {
    //expression inside if statement must evaluate to a bool
    let number = 6;
    if number % 4 == 0 {
        println!("The number is divisible by 4");
    } else if number % 2 == 0 {
        println!("The number is divisible by 2");
    } else {
        println!("The number is not divisible by 2 or 4");
    }

    //if statements are expressions themselves
    let condition = true;
    let number = if condition { 5 } else { 6 }; //types cannot be mismatched
    println!("The number is {number}");

    //loop is infinite, you can use break or continue statements
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }; //semicolon for counter
    println!("The result is {result}");

    /* Since break and continute refer to the innermost loop at that point,
    you can use labels to disambiguate loops using a single quote */
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            };
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count: {count}");

    let mut number = 3;

    //while loops are much clearer than loop, if/else, and break
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!");

    //for loops can be used to iterate through an array easily
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is {element}");
    }

    //countdown but with a for loop
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!");

    let n = 10;
    println!("fibonacci({n}) returned {}", fibonacci(n));
}

fn fibonacci(n: i32) -> i32 {
    let mut last = 1;
    let mut second_last = 1;
    for _i in 3..=n {
        let temp = last;
        last += second_last;
        second_last = temp;
    }
    last
}
