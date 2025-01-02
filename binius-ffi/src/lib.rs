use hs_bindgen::*;
use binius_circuits::{arithmetic, builder::ConstraintSystemBuilder, unconstrained::variable_u128};
use binius_core::{
    constraint_system,
    fiat_shamir::HasherChallenger,
    tower::CanonicalTowerFamily,
};
use binius_field::{
    arch::OptimalUnderlier, BinaryField128b, BinaryField1b, BinaryField8b,
};
use binius_hal::make_portable_backend;
use binius_hash::{GroestlDigestCompression, GroestlHasher};
use binius_math::DefaultEvaluationDomainFactory;
use groestl_crypto::Groestl256;

const ROWS: usize = 7;

/*
-- TODO / FIXME
-- Currently program is fixed (it is a two u32 values addition) and defined
-- in binius-ffi/src/lib.rs. We pass actual values of that program to the 'prove' function.
--
-- Eventually we should use a whole program as an input to a 'prove' FFI function.
-- This program has to be represented as bytes, which are actually a serialized
-- system of constraints and witness defined by Binius.
--
-- The output of 'prove' FFI function is a serialized Binius proof. During verification, that might happen in
-- a different environment (for example on chain), the proof has to be deserialized and processed.
*/

#[hs_bindgen]
fn prove(x: u32, y: u32) -> Vec<u8> {
    let allocator = bumpalo::Bump::new();
    let mut builder =
        ConstraintSystemBuilder::<OptimalUnderlier, BinaryField128b>::new_with_witness(&allocator);

    let x_id = variable_u128::<_, _, BinaryField1b>(&mut builder, "x", ROWS, x as u128).unwrap();
    let y_id = variable_u128::<_, _, BinaryField1b>(&mut builder, "y", ROWS, y as u128).unwrap();

    arithmetic::u32::add(&mut builder, "x + y", x_id, y_id, arithmetic::Flags::Unchecked).unwrap();

    let witness = builder.take_witness().unwrap();
    let cs = builder.build().unwrap();

    let domain_factory = DefaultEvaluationDomainFactory::default();
    let backend = make_portable_backend();

    let proof = constraint_system::prove::<
        OptimalUnderlier,
        CanonicalTowerFamily,
        BinaryField8b,
        _,
        _,
        GroestlHasher<BinaryField128b>,
        GroestlDigestCompression<BinaryField8b>,
        HasherChallenger<Groestl256>,
        _,
    >(&cs, 1usize, 100usize, witness, &domain_factory, &backend).unwrap();

    // Verify just to ensure that proof is valid
    constraint_system::verify::<
        OptimalUnderlier,
        CanonicalTowerFamily,
        _,
        GroestlHasher<BinaryField128b>,
        GroestlDigestCompression<BinaryField8b>,
        HasherChallenger<Groestl256>,
    >(&cs, 1usize, 100usize, vec![], proof.clone()).unwrap();

    // return serialized proof as a regular vector of bytes
    [proof.transcript, proof.advice].concat().to_vec()
}
