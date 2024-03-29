To compile the C code to generate the library.

`gcc -shared -o libmylib.so mylib.c`

To compile the Rust code to generate the library.

`rustc --crate-type cdylib mylib.rs`



