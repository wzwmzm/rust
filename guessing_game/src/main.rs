fn main() {
    println!("Hello, world!");

    let x=another_function();
    println!("{}\n",x);

    for number in 1..4 {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function()->i32 {
    78;
    println!("Another function.");
    678
}