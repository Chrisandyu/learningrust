#[allow(unused_variables)]
fn main() {
    /*
    The Stack: You can only push and pop, memory pushed must be a fixed size
    The Heap: Things that have unknown sizes at compile time gets put here.
              A pointer to the thing can be put on the stack
    */

    {
                            //s is not valid here, not declared
        let s = "hello";   //s is valid from this point
    }
                            //the scope is now over, s is not longer valid

    //String literals have a fixed size and are immutable
    //String types are stored on the heap and are mutable
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    {
        let s = String::from("hello"); //allocate is called here by us
    }
                                        //drop() is automatically called after scope ends

    /*
    Issue:
    The entire s1 string is not copied on the heap because it is costly(deep copy).
    Instead, another pointer to the string is made (shallow copy).
    However, when s1 and s2 go out of scope the char array on the heap
    will be auto dropped twice.

    Solution:
    Instead, s1 is no longer valid and ownership of the string on the heap
    is given to s2. s1 was moved into s2 and can no longer be accessed.
    */
    let s1 = String::from("hello");
    let s2 = s1;
    //Invalid access: println!("The value of s1 is {s1}");

    //Data can still be deep copied, though its not automatic
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{s1}, {s2}");

    //primitives with the Copy trait are auto copied as they're on the stack
    let x = 5;
    let y = x;

    //passing a value to a function is similar to variable assignment:
    {
        let s = String::from("hello");  //s comes into scope
        takes_ownership(s);             //s's value moves into the function ...
                                        //... and so is no longer valid here

        let x = 5;      //x comes into scope
        makes_copy(x);  //x would move into the function, but i32 has
                        //the Copy trait so its ok to still use x after  

    }  //Here, x goes out of scope, then s. But because the value of s is moved
       //nothing special happens.

    fn takes_ownership(some_string: String) {
        println!("{some_string}");
    }  //Here, some_string goes out of scope and 'drop' is called.

    fn makes_copy(some_integer: i32) {
        println!("{some_integer}");
    } //Here, some_integer goes out of scope. Nothing special happens.

    

}
