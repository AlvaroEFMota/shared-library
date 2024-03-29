extern crate libloading;

use libloading::{Library, Symbol};

fn main() {
    // Load the dynamic library
    unsafe {
        let lib = Library::new("../libmylib.so").unwrap();
        // Load the function symbol from the library
        let func: Symbol<unsafe extern "C" fn(i32, i32) -> i32> = match lib.get(b"add") {
            Ok(func) => func,
            Err(_) => {
                println!("Failed to load function");
                return;
            }
        };

        // Call the function
        let result = func(0, 6);

        println!("Result: {}", result);
    }
}
