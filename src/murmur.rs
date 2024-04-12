// taken from murmur3 wiki page
// chatgpt help
// and https://github.com/judwhite/Grassfed.MurmurHash3/blob/master/Grassfed.MurmurHash3/MurmurHash3.cs

use bincode;
use byteorder::{ByteOrder, LittleEndian};
use serde::Serialize;
use std::io::Result;

pub fn hash_anything<T: Serialize>(item: &T, seed: u32) -> Result<u128> {
    let bytes = bincode::serialize(item).expect("Can't convert this item");
    hash_mumur3(bytes, seed)
}

pub fn hash_mumur3<T: AsRef<[u8]>>(key: T, seed: u32) -> Result<u128> {
    const C1: u64 = 0x87c37b91114253d5;
    const C2: u64 = 0x4cf5ad432745937f;

    let mut h_seed1 = seed as u64;
    let mut h_seed2 = seed as u64;

    let key_data = key.as_ref();
    let chunk_size = 16;

    let mut num_bytes = 0;
    for chunk in key_data.chunks(chunk_size) {
        if chunk.len() == chunk_size {
            // converting to little endian to convert to x64 architecture
            let mut k1 = LittleEndian::read_u64(&chunk[0..8]);
            let mut k2 = LittleEndian::read_u64(&chunk[8..chunk.len()]);

            // Mix k1
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(31);
            k1 = k1.wrapping_mul(C2);
            h_seed1 ^= k1;
            h_seed1 = h_seed1.rotate_left(27);
            h_seed1 = h_seed1.wrapping_add(h_seed2);
            h_seed1 = h_seed1.wrapping_mul(5).wrapping_add(0x52dce729);

            // Mix k2
            k2 = k2.wrapping_mul(C2);
            k2 = k2.rotate_left(33);
            k2 = k2.wrapping_mul(C1);
            h_seed2 ^= k2;
            h_seed2 = h_seed2.rotate_left(31);
            h_seed2 = h_seed2.wrapping_add(h_seed1);
            h_seed2 = h_seed2.wrapping_mul(5).wrapping_add(0x38495ab5);

            num_bytes += 16;
        } else {
            // tail

            let remaining = chunk.len();
            num_bytes += remaining;
            let mut k1: u64 = 0;
            let mut k2: u64 = 0;

            if remaining >= 15 {
                k2 ^= (chunk[14] as u64) << 48;
            }
            if remaining >= 14 {
                k2 ^= (chunk[13] as u64) << 40;
            }
            if remaining >= 13 {
                k2 ^= (chunk[12] as u64) << 32;
            }
            if remaining >= 12 {
                k2 ^= (chunk[11] as u64) << 24;
            }
            if remaining >= 11 {
                k2 ^= (chunk[10] as u64) << 16;
            }
            if remaining >= 10 {
                k2 ^= (chunk[9] as u64) << 8;
            }
            if remaining >= 9 {
                k2 ^= chunk[8] as u64;
                k2 = k2.wrapping_mul(C2);
                k2 = k2.rotate_left(33);
                k2 = k2.wrapping_mul(C1);
                h_seed2 ^= k2;
            }

            if remaining >= 8 {
                k1 ^= LittleEndian::read_u64(&chunk[0..8]);
            } else {
                if remaining >= 7 {
                    k1 ^= (chunk[6] as u64) << 48;
                }
                if remaining >= 6 {
                    k1 ^= (chunk[5] as u64) << 40;
                }
                if remaining >= 5 {
                    k1 ^= (chunk[4] as u64) << 32;
                }
                if remaining >= 4 {
                    k1 ^= (chunk[3] as u64) << 24;
                }
                if remaining >= 3 {
                    k1 ^= (chunk[2] as u64) << 16;
                }
                if remaining >= 2 {
                    k1 ^= (chunk[1] as u64) << 8;
                }
                if remaining >= 1 {
                    k1 ^= chunk[0] as u64;
                }
            }

            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(31);
            k1 = k1.wrapping_mul(C2);
            h_seed1 ^= k1;

            // final mixes
            h_seed1 ^= num_bytes as u64;
            h_seed2 ^= num_bytes as u64;

            h_seed1 = h_seed1.wrapping_add(h_seed2);
            h_seed2 = h_seed2.wrapping_add(h_seed1);

            h_seed1 = fmix64(h_seed1);
            h_seed2 = fmix64(h_seed2);

            h_seed1 = h_seed1.wrapping_add(h_seed2);
            h_seed2 = h_seed2.wrapping_add(h_seed1);

            let x = ((h_seed2 as u128) << 64) | (h_seed1 as u128);
            return Ok(x);
        }
    }
    Ok(1 as u128)
}

fn fmix64(mut k: u64) -> u64 {
    k ^= k >> 33;
    k = k.wrapping_mul(0xff51afd7ed558ccd);
    k ^= k >> 33;
    k = k.wrapping_mul(0xc4ceb9fe1a85ec53);
    k ^= k >> 33;

    k
}
