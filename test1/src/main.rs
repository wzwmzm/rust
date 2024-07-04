//use std::sync::Mutex;


fn main() {
    let names  = String::from("Sunface, Jack, Allen");
    //    let v = NAMES.lock().unwrap();
    println!("{}", names);

    let ptr: *const i32 = &42;
//unsafe {
    println!("{}", *ptr); 
} 