#[no_mangle]
pub extern "C" fn multiply(v1: i32, v2: i32, v3: i32) -> i32 {
    v1 * v2 *v3
}

#[no_mangle]
pub extern "C" fn say_hello()  {
    println!("hello")
}
