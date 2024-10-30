fn main() {
    /* by default variables are immutable
    types are implicitly defined */
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    //constants can't be mutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    //shadowing: new var with same name as old one
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }
    //inner shadowing ends here
    println!("The value of x in the outer scope is {x}");

    //shadowing also lets you change types (doesn't work with mut)
    let spaces = "     ";
    let spaces = spaces.len();
    println!("Number of spaces: {spaces}");
}
