#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

#[macro_use]
mod vga_buffer;

extern crate volatile;
extern crate rlibc;
extern crate spin;

#[no_mangle]
pub extern fn rust_main() {

    vga_buffer::clear_screen();
    println!("{}", { println!("inner"); "outer" });

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}

