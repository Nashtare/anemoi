pub use ark_bn254::Fq as Felt;
pub use ark_ff::BigInteger256;
use ark_ff::Field;

mod sbox;

/// An instantiation of Anemoi with state width 2 and
/// rate 1 aimed at providing 128 bits security.
pub mod anemoi_2_1;

/// An instantiation of Anemoi with state width 4 and
/// rate 3 aimed at providing 128 bits security.
pub mod anemoi_4_3;

// HELPER FUNCTION
// ================================================================================================

#[inline(always)]
fn mul_by_generator(x: &Felt) -> Felt {
    let x2 = x.double();

    x2 + x
}
