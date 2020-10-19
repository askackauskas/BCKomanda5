use ethereum_types::H256;
use hex_literal::hex;
use sha2::{Digest as _, Sha256};

#[rustfmt::skip]
pub const ZERO_HASHES: [H256; 32] = [
    H256(hex!("0000000000000000000000000000000000000000000000000000000000000000")),
    H256(hex!("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b")),
    H256(hex!("db56114e00fdd4c1f85c892bf35ac9a89289aaecb1ebd0a96cde606a748b5d71")),
    H256(hex!("c78009fdf07fc56a11f122370658a353aaa542ed63e44c4bc15ff4cd105ab33c")),
    H256(hex!("536d98837f2dd165a55d5eeae91485954472d56f246df256bf3cae19352a123c")),
    H256(hex!("9efde052aa15429fae05bad4d0b1d7c64da64d03d7a1854a588c2cb8430c0d30")),
    H256(hex!("d88ddfeed400a8755596b21942c1497e114c302e6118290f91e6772976041fa1")),
    H256(hex!("87eb0ddba57e35f6d286673802a4af5975e22506c7cf4c64bb6be5ee11527f2c")),
    H256(hex!("26846476fd5fc54a5d43385167c95144f2643f533cc85bb9d16b782f8d7db193")),
    H256(hex!("506d86582d252405b840018792cad2bf1259f1ef5aa5f887e13cb2f0094f51e1")),
    H256(hex!("ffff0ad7e659772f9534c195c815efc4014ef1e1daed4404c06385d11192e92b")),
    H256(hex!("6cf04127db05441cd833107a52be852868890e4317e6a02ab47683aa75964220")),
    H256(hex!("b7d05f875f140027ef5118a2247bbb84ce8f2f0f1123623085daf7960c329f5f")),
    H256(hex!("df6af5f5bbdb6be9ef8aa618e4bf8073960867171e29676f8b284dea6a08a85e")),
    H256(hex!("b58d900f5e182e3c50ef74969ea16c7726c549757cc23523c369587da7293784")),
    H256(hex!("d49a7502ffcfb0340b1d7885688500ca308161a7f96b62df9d083b71fcc8f2bb")),
    H256(hex!("8fe6b1689256c0d385f42f5bbe2027a22c1996e110ba97c171d3e5948de92beb")),
    H256(hex!("8d0d63c39ebade8509e0ae3c9c3876fb5fa112be18f905ecacfecb92057603ab")),
    H256(hex!("95eec8b2e541cad4e91de38385f2e046619f54496c2382cb6cacd5b98c26f5a4")),
    H256(hex!("f893e908917775b62bff23294dbbe3a1cd8e6cc1c35b4801887b646a6f81f17f")),
    H256(hex!("cddba7b592e3133393c16194fac7431abf2f5485ed711db282183c819e08ebaa")),
    H256(hex!("8a8d7fe3af8caa085a7639a832001457dfb9128a8061142ad0335629ff23ff9c")),
    H256(hex!("feb3c337d7a51a6fbf00b9e34c52e1c9195c969bd4e7a0bfd51d5c5bed9c1167")),
    H256(hex!("e71f0aa83cc32edfbefa9f4d3e0174ca85182eec9f3a09f6a6c0df6377a510d7")),
    H256(hex!("31206fa80a50bb6abe29085058f16212212a60eec8f049fecb92d8c8e0a84bc0")),
    H256(hex!("21352bfecbeddde993839f614c3dac0a3ee37543f9b412b16199dc158e23b544")),
    H256(hex!("619e312724bb6d7c3153ed9de791d764a366b389af13c58bf8a8d90481a46765")),
    H256(hex!("7cdd2986268250628d0c10e385c58c6191e6fbe05191bcc04f133f2cea72c1c4")),
    H256(hex!("848930bd7ba8cac54661072113fb278869e07bb8587f91392933374d017bcbe1")),
    H256(hex!("8869ff2c22b28cc10510d9853292803328be4fb0e80495e8bb8d271f5b889636")),
    H256(hex!("b5fe28e79f1b850f8658246ce9b6a1e7b49fc06db7143e8fe0b4f2b0c5523a5c")),
    H256(hex!("985e929f70af28d0bdd1a90a808f977f597c7c778c489e98d3bd8910d31ac0f7")),
];

#[must_use]
pub fn hash(bytes: impl AsRef<[u8]>) -> H256 {
    H256(Sha256::digest(bytes.as_ref()).into())
}

#[must_use]
pub fn concatenate_and_hash(left: impl AsRef<[u8]>, right: impl AsRef<[u8]>) -> H256 {
    H256(Sha256::new().chain(left).chain(right).finalize().into())
}

// `function`               | `function(1)`
// ------------------------ | --------------------------------------------------------------------
// `H256::from_low_u64_le`  | `0x0000000000000000000000000000000000000000000000000100000000000000`
// `hashing::hash_from_u64` | `0x0100000000000000000000000000000000000000000000000000000000000000`
#[must_use]
pub fn hash_from_u64(number: u64) -> H256 {
    let mut h256 = H256::zero();
    h256[..core::mem::size_of::<u64>()].copy_from_slice(&number.to_le_bytes());
    h256
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn higher_zero_hashes_are_calculated_from_lower_ones() {
        let (_, lower_hashes) = ZERO_HASHES.split_last().expect("ZERO_HASHES is not empty");
        let (_, higher_hashes) = ZERO_HASHES.split_first().expect("ZERO_HASHES is not empty");

        assert!(lower_hashes
            .iter()
            .map(|lower| concatenate_and_hash(lower, lower))
            .eq(higher_hashes.iter().copied()));
    }
}
