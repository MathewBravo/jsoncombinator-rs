use serde_json::Value;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

pub fn find_matches(a: String, b: String) {
    let file_a = File::open(a).unwrap();
    let file_b = File::open(b).unwrap();

    let reader_a = BufReader::new(file_a);
    let reader_b = BufReader::new(file_b);

    let json_a: Value = serde_json::from_reader(reader_a).unwrap();
    let json_b: Value = serde_json::from_reader(reader_b).unwrap();

    let keys_a = get_keys(&json_a);
    let keys_b = get_keys(&json_b);

    let common_keys: HashSet<_> = keys_a.intersection(&keys_b).collect();

    println!("Common keys: {:?}", common_keys);
}

fn get_keys(json: &Value) -> HashSet<String> {
    let mut keys = HashSet::new();

    match json {
        Value::Object(map) => {
            for (key, value) in map {
                keys.insert(key.to_owned());
                let nested_keys = get_keys(value);
                keys.extend(nested_keys);
            }
        }
        Value::Array(arr) => {
            for value in arr {
                let nested_keys = get_keys(value);
                keys.extend(nested_keys);
            }
        }
        _ => {}
    }

    keys
}
