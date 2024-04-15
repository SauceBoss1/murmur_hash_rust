// #[cfg(test)]
// mod hash_test {
//     use murmur_hash_rust::{hash_anything, hash_mumur3};

//     #[test]
//     fn test_hash() {
//         let key = "Hello World!".to_string();
//         let key2 = [32u8];
//         let seed = 42;
//         let hash_res = hash_mumur3(key, seed);
//         let hash_all_res = hash_anything(&key2, seed);

//         if let Ok(hash_num) = hash_res {
//             println!("{:?}", hash_num);
//         }

//         if let Ok(hash_num) = hash_all_res {
//             println!("{:?}", hash_num)
//         }
//     }
// }
