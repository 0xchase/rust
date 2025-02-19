// normalize-stderr-test "(the raw bytes of the constant) \(size: [0-9]*, align: [0-9]*\)" -> "$1 (size: $$SIZE, align: $$ALIGN)"
// normalize-stderr-test "( 0x[0-9a-f][0-9a-f] │)? ([0-9a-f][0-9a-f] |__ |╾─*ALLOC[0-9]+(\+[a-z0-9]+)?(<imm>)?─*╼ )+ *│.*" -> " HEX_DUMP"
// normalize-stderr-test "HEX_DUMP\s*\n\s*HEX_DUMP" -> "HEX_DUMP"
#![feature(const_mut_refs, const_refs_to_static)]
#![feature(raw_ref_op)]

use std::sync::Mutex;

// This file checks that our dynamic checks catch things that the static checks miss.
// We do not have static checks for these, because we do not look into function bodies.
// We treat all functions as not returning a mutable reference, because there is no way to
// do that without causing the borrow checker to complain (see the B4/helper test in
// mut_ref_in_final.rs).

static mut BUFFER: i32 = 42;

const fn helper() -> Option<&'static mut i32> { unsafe {
    Some(&mut *std::ptr::addr_of_mut!(BUFFER))
} }
const MUT: Option<&mut i32> = helper(); //~ ERROR it is undefined behavior to use this value
//~^ encountered mutable reference
static MUT_STATIC: Option<&mut i32> = helper(); //~ ERROR it is undefined behavior to use this value
//~^ encountered mutable reference

const fn helper_int2ptr() -> Option<&'static mut i32> { unsafe {
    // Undefined behaviour (integer as pointer), who doesn't love tests like this.
    Some(&mut *(42 as *mut i32))
} }
const INT2PTR: Option<&mut i32> = helper_int2ptr(); //~ ERROR it is undefined behavior to use this value
//~^  encountered a dangling reference
static INT2PTR_STATIC: Option<&mut i32> = helper_int2ptr(); //~ ERROR it is undefined behavior to use this value
//~^  encountered a dangling reference

const fn helper_dangling() -> Option<&'static mut i32> { unsafe {
    // Undefined behaviour (dangling pointer), who doesn't love tests like this.
    Some(&mut *(&mut 42 as *mut i32))
} }
const DANGLING: Option<&mut i32> = helper_dangling(); //~ ERROR encountered dangling pointer
static DANGLING_STATIC: Option<&mut i32> = helper_dangling(); //~ ERROR encountered dangling pointer

// Variant of the real-world case in <https://github.com/rust-lang/rust/issues/120450>.
// Maybe we should allow this in the future (then the rest should move to `const_mut_refs.rs`),
// but for now we reject it.
static mut MUT_ARRAY: &mut [u8] = &mut [42];
static MUTEX: Mutex<&mut [u8]> = Mutex::new(unsafe { &mut *MUT_ARRAY }); //~ ERROR it is undefined behavior to use this value
//~^ encountered mutable reference

fn main() {}
