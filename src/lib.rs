use neptune::poseidon::{Poseidon, PoseidonConstants};
use blstrs::Scalar as Fr;
use ff::{Field, PrimeField};
use generic_array::typenum;

pub(crate) fn scalar_from_u64s(parts: &[u64; 4]) -> Fr {
    let mut le_bytes = [0u8; 32];
    le_bytes[0..8].copy_from_slice(&parts[0].to_le_bytes());
    le_bytes[8..16].copy_from_slice(&parts[1].to_le_bytes());
    le_bytes[16..24].copy_from_slice(&parts[2].to_le_bytes());
    le_bytes[24..32].copy_from_slice(&parts[3].to_le_bytes());
    let mut repr = <Fr as PrimeField>::Repr::default();
    repr.as_mut().copy_from_slice(&le_bytes[..]);
    Fr::from_repr_vartime(repr).expect("u64s exceed BLS12-381 scalar field modulus")
}

#[no_mangle]
extern "C" fn hash_arity_4<'a>(part1: &'a [u64; 4], part2: &'a [u64; 4], part3: &'a [u64; 4], part4: &'a [u64; 4]) -> &'a [u8; 32] {
    let fr1 = scalar_from_u64s(part1);
    let fr2 = scalar_from_u64s(part2);
    let fr3 = scalar_from_u64s(part3);
    let fr4 = scalar_from_u64s(part4);
    let preimage = [fr1, fr2, fr3, fr4];
    let constants = PoseidonConstants::new();

    let mut h = Poseidon::<Fr, typenum::U4>::new_with_preimage(&preimage, &constants);

    let fr_hash = h.hash();
    
    &fr_hash.to_bytes_le()
}

mod test {
    use super::*;

    #[test]
    fn works() {
        assert_eq!(hash_arity_4([0,0,0,1], [0,0,0,2], [0,0,0,3], [0,0,0,4]), [0;32])
    }
}