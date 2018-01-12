pub mod foo {
    #[no_mangle]
    pub static STATIC: [u32; 10] = [1; 10];

    pub fn hello() {}
}

pub fn bar() {
    foo::hello(); // STATIC not present if commented out
}

