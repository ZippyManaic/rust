// ignore-compare-mode-nll
// revisions: base nll
// [nll]compile-flags: -Zborrowck=mir

// Test that we give a note when the old LUB/GLB algorithm would have
// succeeded but the new code (which is stricter) gives an error.

trait Foo<T, U> {}

fn foo(x: &dyn for<'a, 'b> Foo<&'a u8, &'b u8>, y: &dyn for<'a> Foo<&'a u8, &'a u8>) {
    let z = match 22 {
        //[base]~^ ERROR mismatched types
        0 => x,
        _ => y,
        //[nll]~^ ERROR mismatched types
        //[nll]~| ERROR mismatched types
    };
}

fn bar(x: &dyn for<'a, 'b> Foo<&'a u8, &'b u8>, y: &dyn for<'a> Foo<&'a u8, &'a u8>) {
    // Accepted with explicit case:
    let z = match 22 {
        0 => x as &dyn for<'a> Foo<&'a u8, &'a u8>,
        _ => y,
    };
}

fn main() {}
