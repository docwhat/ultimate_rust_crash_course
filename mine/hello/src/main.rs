fn main() {
    let mut x = 5;
    x += 1;
    {
        let x = 99;
        println!("{}", x);
    }
    println!("{}", x);
}
