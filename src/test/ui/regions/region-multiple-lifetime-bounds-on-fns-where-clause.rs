// revisions: base nll
// ignore-compare-mode-nll
//[nll] compile-flags: -Z borrowck=mir

fn a<'a, 'b, 'c>(x: &mut &'a isize, y: &mut &'b isize, z: &mut &'c isize) where 'b: 'a + 'c {
    // Note: this is legal because of the `'b:'a` declaration.
    *x = *y;
    *z = *y;
}

fn b<'a, 'b, 'c>(x: &mut &'a isize, y: &mut &'b isize, z: &mut &'c isize) {
    // Illegal now because there is no `'b:'a` declaration.
    *x = *y;
    //[base]~^ ERROR E0623
    *z = *y; //[base]~ ERROR E0623
}

fn c<'a,'b, 'c>(x: &mut &'a isize, y: &mut &'b isize, z: &mut &'c isize) {
    // Here we try to call `foo` but do not know that `'a` and `'b` are
    // related as required.
    a(x, y, z);
    //[base]~^ ERROR lifetime mismatch [E0623]
}

fn d() {
    // 'a and 'b are early bound in the function `a` because they appear
    // inconstraints:
    let _: fn(&mut &isize, &mut &isize, &mut &isize) = a;
    //~^ ERROR E0308
}

fn e() {
    // 'a and 'b are late bound in the function `b` because there are
    // no constraints:
    let _: fn(&mut &isize, &mut &isize, &mut &isize) = b;
}

fn main() { }
