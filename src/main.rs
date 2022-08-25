fn main() {
    let hello = "hello!";

    println!("Hello, World!");

    let mut i = 1;


    loop {
        if i > 10 {
            println!("{}", hello);
            drop(hello);
            break;
        } else {
            i = i+1;
            println!("{}", i)
        }
    }
}