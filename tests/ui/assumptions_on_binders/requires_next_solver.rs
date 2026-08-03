//@ revisions: bare coherence
//@[bare] compile-flags: -Zassumptions-on-binders
//@[coherence] compile-flags: -Znext-solver=coherence -Zassumptions-on-binders

// Regression test for #160256.
//
// `-Zassumptions-on-binders` routes region solving through the next solver's
// region-constraint machinery, which is only wired up when the next solver is enabled
// *globally*. Passing it without `-Znext-solver=globally` used to reach
// `assert!(self.next_trait_solver())` during regionck and ICE. It is now rejected as an
// invalid flag combination before analysis runs. `-Znext-solver=coherence` only enables the
// next solver during coherence, so it is rejected the same way as passing no next-solver flag.

trait Foo<T> {}

fn foo<'a, 'b>(_a: impl for<'c: 'a + 'b> Foo<&'c usize>) {}

fn main() {}

//[bare]~? ERROR `-Zassumptions-on-binders` requires `-Znext-solver=globally`
//[coherence]~? ERROR `-Zassumptions-on-binders` requires `-Znext-solver=globally`
