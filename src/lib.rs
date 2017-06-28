#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

mod vga_buffer;

extern crate volatile;
extern crate rlibc;


#[no_mangle]
pub extern fn rust_main() {

    vga_buffer::print_something("What the fuck");
    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}

