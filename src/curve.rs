use crate::Fq;
use crate::Fr;
use core::cmp;
use core::fmt::Debug;
use core::iter::Sum;
use core::ops::{Add, Mul, Neg, Sub};
use ff::{Field, PrimeField};
use group::Curve;
use group::{prime::PrimeCurveAffine, Group as _, GroupEncoding};
use halo2curves::{Coordinates, CurveAffine, CurveAffineExt, CurveExt};
use rand::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use crate::new_curve_impl;
use halo2curves::{
    batch_add, impl_add_binop_specify_output, impl_binops_additive,
    impl_binops_additive_specify_output, impl_binops_multiplicative,
    impl_binops_multiplicative_mixed, impl_sub_binop_specify_output,
};

new_curve_impl!(
    (pub),
    Grumpkin,
    GrumpkinAffine,
    GrumpkinCompressed,
    Fq,
    Fr,
    (GRUMPKIN_GENERATOR_X,GRUMPKIN_GENERATOR_Y),
    GRUMPKIN_B,
    "grumpkin",
);

new_curve_impl!(
    (pub),
    BN256,
    BN256Affine,
    BN256Compressed,
    Fr,
    Fq,
    (BN256_GENERATOR_X,BN256_GENERATOR_Y),
    BN256_B,
    "bn256",
);

impl Grumpkin {
    fn endomorphism_base(&self) -> Self {
        unimplemented!();
    }
}

impl CurveAffineExt for GrumpkinAffine {
    batch_add!();
}

impl BN256 {
    fn endomorphism_base(&self) -> Self {
        unimplemented!();
    }
}

impl CurveAffineExt for BN256Affine {
    batch_add!();
}

const GRUMPKIN_GENERATOR_X: Fq = Fq::one();
const GRUMPKIN_GENERATOR_Y: Fq = Fq([
    0x11b2dff1448c41d8,
    0x23d3446f21c77dc3,
    0xaa7b8cf435dfafbb,
    0x14b34cf69dc25d68,
]);
const GRUMPKIN_B: Fq = Fq([
    0xdd7056026000005a,
    0x223fa97acb319311,
    0xcc388229877910c0,
    0x34394632b724eaa,
]);

const BN256_GENERATOR_X: Fr = Fr::one();
const BN256_GENERATOR_Y: Fr = Fr::from_raw([2, 0, 0, 0]);
const BN256_B: Fr = Fr::from_raw([3, 0, 0, 0]);

impl group::cofactor::CofactorGroup for Grumpkin {
    type Subgroup = Grumpkin;

    fn clear_cofactor(&self) -> Self {
        *self
    }

    fn into_subgroup(self) -> CtOption<Self::Subgroup> {
        CtOption::new(self, 1.into())
    }

    fn is_torsion_free(&self) -> Choice {
        1.into()
    }
}

impl group::cofactor::CofactorGroup for BN256 {
    type Subgroup = BN256;

    fn clear_cofactor(&self) -> Self {
        *self
    }

    fn into_subgroup(self) -> CtOption<Self::Subgroup> {
        CtOption::new(self, 1.into())
    }

    fn is_torsion_free(&self) -> Choice {
        1.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Grumpkin, BN256};

    #[test]
    fn test_curve() {
        crate::tests::curve::curve_tests::<Grumpkin>();
        crate::tests::curve::curve_tests::<BN256>();
    }
}
