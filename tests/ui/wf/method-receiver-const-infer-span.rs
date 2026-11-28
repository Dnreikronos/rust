//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@ [next] compile-flags: -Znext-solver

// Regression test for <https://github.com/rust-lang/rust/issues/165097>.
//
// `check_method_receiver` previously normalized the entire method signature,
// `self_ty`, and `receiver_ty` with `DUMMY_SP`, causing an ICE
// (`assertion failed: !origin.span.is_dummy()`) when reporting an ambiguous
// const inference variable in `extract_inference_diagnostics_data`.

use std::ops::Deref;

trait Foo {}

impl<'a, const N: u8> Deref for dyn Foo {
    //~^ ERROR not all trait items implemented, missing: `Target`
    //~| ERROR the const parameter `N` is not constrained by the impl trait, self type, or predicates
    fn deref(&self) -> &'a Self::Target {
        //~^ ERROR type annotations needed
        //~| ERROR type annotations needed
        //[next]~^^^ ERROR type annotations needed
        //[next]~| ERROR type annotations needed
        //[next]~| ERROR type annotations needed
        //[next]~| ERROR type annotations needed
        todo!();
    }
}

trait Bar {
    type Assoc: ?Sized;
}

impl<const N: u8> Bar for dyn Foo {}
//~^ ERROR not all trait items implemented, missing: `Assoc`
//~| ERROR the const parameter `N` is not constrained by the impl trait, self type, or predicates

trait Baz {
    fn baz(&self);
}

impl<const N: u8> Baz for <dyn Foo as Bar>::Assoc {
    //~^ ERROR the const parameter `N` is not constrained by the impl trait, self type, or predicates
    fn baz(&self) {}
    //~^ ERROR type annotations needed
    //~| ERROR type annotations needed
    //[next]~^^^ ERROR type annotations needed
    //[next]~| ERROR type annotations needed
    //[next]~| ERROR type annotations needed
    //[next]~| ERROR type annotations needed
    //[next]~| ERROR type annotations needed
    //[next]~| ERROR type annotations needed
}

fn main() {}
