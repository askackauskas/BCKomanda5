use core::convert::{TryFrom as _, TryInto as _};

use types::{config::Config, primitives::H256};

const SEED_SIZE: usize = 32;
const ROUND_SIZE: usize = 1;
const POSITION_WINDOW_SIZE: usize = 4;
const PIVOT_VIEW_SIZE: usize = SEED_SIZE + ROUND_SIZE;
const TOTAL_SIZE: usize = SEED_SIZE + ROUND_SIZE + POSITION_WINDOW_SIZE;

// Based on <https://github.com/protolambda/eth2-shuffle/tree/fd840f1036c1f8f6d7625ffe6ff4d9c60f942876>.
pub fn shuffle<T, C: Config>(input: &mut [T], seed: H256) {
    if input.len() <= 1 {
        return;
    }

    let list_size = u64::try_from(input.len()).expect("input length should fit in u64");

    let mut buf = [0; TOTAL_SIZE];
    buf[..SEED_SIZE].copy_from_slice(seed.as_bytes());

    for r in (0..C::SHUFFLE_ROUND_COUNT).rev() {
        buf[SEED_SIZE] = r;

        let pivot_bytes = hashing::hash(&buf[..PIVOT_VIEW_SIZE])[..core::mem::size_of::<u64>()]
            .try_into()
            .expect("slice being converted has same size as u64");
        let pivot = usize::try_from(u64::from_le_bytes(pivot_bytes) % list_size)
            .expect("pivot should fit in usize");

        let mirror = (pivot + 1) >> 1;

        set_position_window(&mut buf, pivot);
        let mut source = hashing::hash(&buf[..]);
        let mut byte_v = source[(pivot & 0xff) >> 3];

        for i in 0..mirror {
            let j = pivot - i;

            if j & 0xff == 0xff {
                set_position_window(&mut buf, j);
                source = hashing::hash(&buf[..]);
            }

            if j & 0x7 == 0x7 {
                byte_v = source[(j & 0xff) >> 3];
            }

            let bit_v = (byte_v >> (j & 0x7)) & 0x1;

            if bit_v == 1 {
                input.swap(i, j);
            }
        }

        let mirror = (pivot + input.len() + 1) >> 1;
        let end = input.len() - 1;

        set_position_window(&mut buf, end);
        source = hashing::hash(&buf[..]);
        byte_v = source[(end & 0xff) >> 3];

        for i in (pivot + 1)..mirror {
            let j = end - i + pivot + 1;

            if j & 0xff == 0xff {
                set_position_window(&mut buf, j);
                source = hashing::hash(&buf[..]);
            }

            if j & 0x7 == 0x7 {
                byte_v = source[(j & 0xff) >> 3];
            }

            let bit_v = (byte_v >> (j & 0x7)) & 0x1;

            if bit_v == 1 {
                input.swap(i, j);
            }
        }
    }
}

fn set_position_window(buf: &mut [u8], unshifted_value: usize) {
    let value = u32::try_from(unshifted_value >> 8).expect("shifted value should fit in u32");
    buf[PIVOT_VIEW_SIZE..].copy_from_slice(&value.to_le_bytes());
}
