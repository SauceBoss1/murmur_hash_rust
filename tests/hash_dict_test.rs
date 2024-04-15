use murmur_hash_rust::HashDict;
fn create_test_dict_i32(size: i32) -> HashDict<i32, i32> {
    let mut d: HashDict<i32, i32> = HashDict::new(1000000, 69);
    for i in 0..size {
        d.insert(i, i.pow(2));
    }
    return d;
}

#[cfg(test)]
mod hash_dict_test {
    use murmur_hash_rust::{hash_dict, HashDict};

    use crate::create_test_dict_i32;

    #[test]
    fn test_insert() {
        let mut d: HashDict<i32, i32> = HashDict::new(3, 42);
        for i in 0..100 {
            d.insert(i, i.pow(2));
        }
    }

    #[test]
    fn test_get() {
        let d = create_test_dict_i32(1000);
        let key = 999;
        if let Some(res) = d.get(&key) {
            println!("{res}");
            assert_eq!(res, key.pow(2))
        } else {
            println!("nothing here");
        }
    }

    #[test]
    fn test_delete() {
        let mut d = create_test_dict_i32(1000);
        d.delete(&102);
        if let Some(v) = d.get(&102) {
            println!("{v}");
        } else {
            println!("NOTHING");
        }
    }

    #[test]
    fn test_pop() {
        let mut d = create_test_dict_i32(1000);
        if let Some(v) = d.pop(&32) {
            println!("popped val: {v}");
        }

        if d.get(&32).is_none() {
            println!("deleted");
        }
        assert_eq!(d.get(&32), None);
    }

    #[test]
    fn test_pop_all() {
        let s = 1000;
        let mut d = create_test_dict_i32(s);
        for i in 0..s {
            let item = d.pop(&i);
            if let Some(item) = item {
                println!("{item} {i}");
            }
        }
    }

    #[test]
    fn test_dict_strings() {
        let mut dict: HashDict<String, i32> = HashDict::new(10, 42);
        dict.insert("Hello".to_string(), 1)
            .insert("World!".to_string(), 2);

        if let Some(v) = dict.get(&"Hello".to_string()) {
            assert_eq!(v, 1);
        }

        if let Some(v) = dict.get(&"World!".to_string()) {
            assert_eq!(v, 2);
        }
    }

    #[test]
    fn test_macro() {
        let table = hash_dict![10, 42,
            "hi".to_string() => 10,
            "there".to_string() => 11,
        ];

        if let Some(v) = table.get(&"hi".to_string()) {
            println!("{v}");
        }
    }

    #[test]
    fn test_hash_dict_macro() {
        let tablt = hash_dict![200, 3,
            "3" => "hi",
            "2" => "halo",
            "1" => "hola"
        ];

        if let Some(v) = tablt.get(&"3") {
            assert_eq!(v, "hi");
        } else {
            panic!("None was returned");
        }
    }

    #[test]
    fn test_iter() {
        let table = create_test_dict_i32(500);
        println!("{}", table.len());

        let counter = table.iter().count();

        assert_eq!(table.len(), counter)
    }

    #[test]
    fn test_iter_sort() {
        let t = create_test_dict_i32(100);
        for (k, v) in t.iter() {
            println!("{k} => {v}");
        }
    }

    #[test]
    fn test_mut_get() {
        let mut dict = create_test_dict_i32(1000);
        let key = 100;
        println!("{}", dict.get(&key).unwrap());

        dict.get_mut(&key, |v| {
            *v = 0;
        });

        println!("{}", dict.get(&key).unwrap());
    }
}
