fn main() {
    let s = String::from("hello");

    append(&s);

    println!("The content of 's' is '{s}'.");
}

fn append(some_string: &String) {
    some_string.push_str(", world");
}
