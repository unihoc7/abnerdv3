fn main() {
    let mut x = 42;
    let r = &x;
    x = 13;
    
    println!("r = {r}");
}
