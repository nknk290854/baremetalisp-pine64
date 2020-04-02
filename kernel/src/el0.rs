use crate::slab;
use crate::parser;

#[no_mangle]
pub fn el0_entry() -> ! {
    // initialize memory allocator
    slab::init();

    parser::parse("(add 1 2)");

    let p = 0x400000000 as *mut u64;
    unsafe { *p = 10 };

    loop{}
}