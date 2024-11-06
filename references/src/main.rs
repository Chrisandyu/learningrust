#[allow(clippy::ptr_arg)]
#[allow(unused_variables)]
#[allow(clippy::borrowed_box)]
#[allow(clippy::explicit_auto_deref)]

fn main() {
    // returning ownership is tedious, so references are used
    let s1 = String::from("hello");
    let len = calculate_length(&s1); 
    println!("The length of {s1} is {len}");

    fn calculate_length(s: &String) -> usize { //s is a reference to a String
        s.len()
    } // Here, s goes out of scope. But because it doesn't have ownership of what
      // it refers to so nothing is dropped

    //a reference is a type of pointer!!!
    //They allow you to use a variable more than once


    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2);                    //ampersands
    let s = format!("{} {}", m1, m2);

    //g1 points to m1 on the stack which points to the box on the heap
    fn greet(g1: &String, g2: &String){ //also ampersands in signature
        println!("{} {}!", g1, g2);
    }
    //here, no heap data is gone but stack frames for g1 and g2 are

    
    //dereferences
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x; //a = 1
    *x += 1;

    let r1: &Box<i32> = &x; //points to x on the stack
    let b: i32 = **r1; //two dereferences gets the heap value

    let r2: &i32 = &*x; //points to the heap value
    let c: i32 = *r2;  //c = 2



    //rust implicity inserts references and dereferences
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x);

}