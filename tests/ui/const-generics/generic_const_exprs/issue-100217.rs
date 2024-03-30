//@ build-pass

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

trait TraitOne {
    const MY_NUM: usize;
    type MyErr: std::fmt::Debug;

    fn do_one_stuff(arr: [u8; Self::MY_NUM]) -> Result<(), Self::MyErr>;
}

trait TraitTwo {
    fn do_two_stuff();
}

impl<O: TraitOne> TraitTwo for O
where
    [(); Self::MY_NUM]:,
{
    fn do_two_stuff() {
        O::do_one_stuff([5; Self::MY_NUM]).unwrap()
    }
}

struct Blargotron;

#[derive(Debug)]
struct ErrTy<const N: usize>([(); N]);

impl TraitOne for Blargotron {
    const MY_NUM: usize = 3;
    type MyErr = ErrTy<{ Self::MY_NUM }>;

    fn do_one_stuff(_arr: [u8; Self::MY_NUM]) -> Result<(), Self::MyErr> {
        Ok(())
    }
}

fn main() {
    // FIXME: this test is disabled in the "arbitrary self types v2" branch
    // because it results in an ICE. The ICE appears to be because
    // generic const evaluation is only partially implemented. Probing whether
    // function calls match autoreffed self types unfortunately runs into
    // one of the unimplemented parts - previously we didn't need to do that
    // since the relevant function calls matched by-value reference calls -
    // previously we therefore didn't even probe by-reference function calls
    // but now we need to do so in order to check for some shadowing cases.
    // The specific bit which goes wrong is
    // rustc_trait_selection::traits::const_evaluatable::is_const_evaluatable
    // reporting "Missing value for constant, but no error reported?"
    // Blargotron::do_two_stuff();
}
