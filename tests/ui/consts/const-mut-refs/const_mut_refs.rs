// check-pass
#![feature(const_mut_refs)]

use std::sync::Mutex;

struct Foo {
    x: usize
}

const fn foo() -> Foo {
    Foo { x: 0 }
}

impl Foo {
    const fn bar(&mut self) -> usize {
        self.x = 1;
        self.x
    }

}

const fn baz(foo: &mut Foo) -> usize {
    let x = &mut foo.x;
    *x = 2;
    *x
}

const fn bazz(foo: &mut Foo) -> usize {
    foo.x = 3;
    foo.x
}

// Empty slices get promoted so this passes the static checks.
// Make sure it also passes the dynamic checks.
static MUTABLE_REFERENCE_HOLDER: Mutex<&mut [u8]> = Mutex::new(&mut []);

fn main() {
    let _: [(); foo().bar()] = [(); 1];
    let _: [(); baz(&mut foo())] = [(); 2];
    let _: [(); bazz(&mut foo())] = [(); 3];
}
