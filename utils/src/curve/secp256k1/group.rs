// use ark_ec::models::{ModelParameters, SWModelParameters};
// use ark_ff::{MontFp, Zero};

// use crate::{Fq, Fr};

// #[derive(Clone, Default, PartialEq, Eq)]
// pub struct Parameters;

// pub type SWAffine = SWGroupAffine<Parameters>;
// pub type SWProjective = SWGroupProjective<Parameters>;

// impl ModelParameters for Parameters {
//     type BaseField = Fq;
//     type ScalarField = Fr;

//     /// COFACTOR = 1
//     const COFACTOR: &'static [u64] = &[0x1];

//     /// COFACTOR_INV = COFACTOR^{-1} mod r = 1
//     const COFACTOR_INV: Fr = MontFp!(Fr, "1");
// }

// impl SWModelParameters for Parameters {
//     /// COEFF_A = 0
//     const COEFF_A: Fq = MontFp!(Fq, "0");

//     /// COEFF_B = 7
//     const COEFF_B: Fq = MontFp!(Fq, "7");

//     /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
//     const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
//         (G1_GENERATOR_X, G1_GENERATOR_Y);

//     #[inline(always)]
//     fn mul_by_a(_: &Self::BaseField) -> Self::BaseField {
//         Self::BaseField::zero()
//     }
// }

// /// G1_GENERATOR_X = 55066263022277343669578718895168534326250603453777594175500187360389116729240
// pub const G1_GENERATOR_X: Fq = MontFp!(Fq, "55066263022277343669578718895168534326250603453777594175500187360389116729240");

// /// G1_GENERATOR_Y = 32670510020758816978083085130507043184471273380659243275938904335757337482424
// pub const G1_GENERATOR_Y: Fq = MontFp!(Fq, "32670510020758816978083085130507043184471273380659243275938904335757337482424");
use super::*;
use ark_ec::{
    models::short_weierstrass_jacobian::{GroupAffine, GroupProjective},
    ModelParameters, SWModelParameters,
};
use ark_ff::{field_new, Zero};

#[derive(Copy, Clone, Default, PartialEq, Eq)]

pub struct Secp256k1Parameters;
impl ModelParameters for Secp256k1Parameters {
    type BaseField = Fq;
    type ScalarField = Fr;
}

pub type Affine = GroupAffine<Secp256k1Parameters>;
pub type Projective = GroupProjective<Secp256k1Parameters>;

impl SWModelParameters for Secp256k1Parameters {
    /// COEFF_A = 0
    const COEFF_A: Fq = field_new!(Fq, "0");

    /// COEFF_B = 7
    const COEFF_B: Fq = field_new!(Fq, "7");

    /// COFACTOR = 1
    const COFACTOR: &'static [u64] = &[0x1];

    /// COFACTOR_INV = 1
    const COFACTOR_INV: Fr = field_new!(Fr, "1");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G_GENERATOR_X, G_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }
}
/// G_GENERATOR_X = 55066263022277343669578718895168534326250603453777594175500187360389116729240
pub const G_GENERATOR_X: Fq = field_new!(
    Fq,
    "55066263022277343669578718895168534326250603453777594175500187360389116729240"
);

/// G_GENERATOR_Y = 32670510020758816978083085130507043184471273380659243275938904335757337482424
pub const G_GENERATOR_Y: Fq = field_new!(
    Fq,
    "32670510020758816978083085130507043184471273380659243275938904335757337482424"
);
