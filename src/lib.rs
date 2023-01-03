use neptune::poseidon::{Poseidon, PoseidonConstants};
use blstrs::Scalar as Fr;
use ff::{PrimeField};
use generic_array::typenum;

fn scalar_from_u64s(parts: &[u64; 4]) -> Fr {
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
extern "C" fn hash_arity_4(fr_hash: &mut [u8; 32], part1: &[u64; 4], part2: &[u64; 4], part3: &[u64; 4], part4: &[u64; 4]) {
    let fr1 = scalar_from_u64s(part1);
    let fr2 = scalar_from_u64s(part2);
    let fr3 = scalar_from_u64s(part3);
    let fr4 = scalar_from_u64s(part4);
    let preimage = [fr1, fr2, fr3, fr4];
    let constants = PoseidonConstants::new();

    let mut h = Poseidon::<Fr, typenum::U4>::new_with_preimage(&preimage, &constants);

    *fr_hash = h.hash()
                .to_bytes_le();
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn works() {
        let mut trash = [0;32];
        hash_arity_4(&mut trash, &[0,0,0,0], &[0,0,0,0], &[0,0,0,0], &[0,0,0,0]);
        assert_eq!(trash, [0;32])
    }
}