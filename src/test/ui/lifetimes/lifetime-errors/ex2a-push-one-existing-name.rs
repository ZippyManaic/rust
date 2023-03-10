// revisions: base nll
// ignore-compare-mode-nll
//[nll] compile-flags: -Z borrowck=mir

struct Ref<'a, T: 'a> {
    data: &'a T
}

fn foo<'a>(x: &mut Vec<Ref<'a, i32>>, y: Ref<i32>) {
    x.push(y); //~ ERROR explicit lifetime
}

fn main() { }
