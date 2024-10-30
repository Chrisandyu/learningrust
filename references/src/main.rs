#[allow(clippy::ptr_arg)]
fn main() {
    // returning ownership is tedious, so references are used
    let s1 = String::from("hello");
    let len = calculate_length(&s1); 
    println!("The length of {s1} is {len}");

    fn calculate_length(s: &String) -> usize { //s is a reference to a String
        s.len()
    } // Here, s goes out of scope. But because it doesn't have ownership of what
      // it refers to so nothing is dropped
}