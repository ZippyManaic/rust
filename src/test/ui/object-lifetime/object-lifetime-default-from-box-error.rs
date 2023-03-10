// ignore-compare-mode-nll
// revisions: base nll
// [nll]compile-flags: -Zborrowck=mir

// Test various cases where the defaults should lead to errors being
// reported.

#![allow(dead_code)]

trait SomeTrait {
    fn dummy(&self) { }
}

struct SomeStruct<'a> {
    r: Box<dyn SomeTrait+'a>
}

fn load(ss: &mut SomeStruct) -> Box<dyn SomeTrait> {
    // `Box<SomeTrait>` defaults to a `'static` bound, so this return
    // is illegal.

    ss.r
    //[base]~^ ERROR E0759
    //[nll]~^^ ERROR lifetime may not live long enough
    //[nll]~| ERROR cannot move out of
}

fn store(ss: &mut SomeStruct, b: Box<dyn SomeTrait>) {
    // No error: b is bounded by 'static which outlives the
    // (anonymous) lifetime on the struct.

    ss.r = b;
}

fn store1<'b>(ss: &mut SomeStruct, b: Box<dyn SomeTrait+'b>) {
    // Here we override the lifetimes explicitly, and so naturally we get an error.

    ss.r = b; //~ ERROR explicit lifetime required in the type of `ss` [E0621]
}

fn main() {
}
