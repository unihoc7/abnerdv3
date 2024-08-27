fn main() {
    let s1 = String::from("hello");

    let s2 = takes_and_gives_ownership_back(s1);

    println!("The content of 's1' is {s1}");
    println!("The content of 's2' is {s2}");
}

fn takes_and_gives_ownership_back(s: String) -> String {
    return s; // s is moved out of the function.
}