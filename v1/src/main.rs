#[allow(unused_variables)]
fn main() {
    let s1 = String::from("hello"); // s1 is the owner of the resource (string "hello")
    let s2 = s1; // now s2 owns the resource. s1 is no longer valid

    println!("{s1}, world!"); // compiler doesn't like this
}  // end of scope -> drop trait is called
