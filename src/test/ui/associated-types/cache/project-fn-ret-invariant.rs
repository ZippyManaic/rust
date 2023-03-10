#![feature(unboxed_closures)]
// Test for projection cache. We should be able to project distinct
// lifetimes from `foo` as we reinstantiate it multiple times, but not
// if we do it just once. In this variant, the region `'a` is used in
// an invariant position, which affects the results.

// revisions: ok oneuse transmute krisskross
//[ok] check-pass

// ignore-compare-mode-nll
// FIXME(nll): When stabilizing, this test should be replaced with `project-fn-ret-invariant-nll.rs`
// The two would normally be just revisions, but this test uses revisions heavily, so splitting into
// a separate test is just easier.

#![allow(dead_code, unused_variables)]

use std::marker::PhantomData;

struct Type<'a> {
    // Invariant
    data: PhantomData<fn(&'a u32) -> &'a u32>,
}

fn foo<'a>() -> Type<'a> {
    loop {}
}

fn bar<T>(t: T, x: T::Output) -> T::Output
where
    T: FnOnce<()>,
{
    t()
}

#[cfg(ok)] // two instantiations: OK
fn baz<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
    let a = bar(foo, x);
    let b = bar(foo, y);
    (a, b)
}

#[cfg(oneuse)] // one instantiation: BAD
fn baz<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
    let f = foo; // <-- No consistent type can be inferred for `f` here.
    let a = bar(f, x);
    let b = bar(f, y); //[oneuse]~ ERROR lifetime mismatch [E0623]
    (a, b)
}

#[cfg(transmute)] // one instantiations: BAD
fn baz<'a, 'b>(x: Type<'a>) -> Type<'static> {
    // Cannot instantiate `foo` with any lifetime other than `'a`,
    // since it is provided as input.

    bar(foo, x) //[transmute]~ ERROR E0759
}

#[cfg(krisskross)] // two instantiations, mixing and matching: BAD
fn transmute<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
    let a = bar(foo, y); //[krisskross]~ ERROR E0623
    let b = bar(foo, x);
    (a, b) //[krisskross]~ ERROR E0623
}

fn main() {}
