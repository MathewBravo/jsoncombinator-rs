use serde_json::{json, Value};
use std::fs;

pub fn combine_matches(a: String, b: String, key: String) {
    // Read the contents of file A and file B
    let file_a_contents = fs::read_to_string(a).expect("Unable to read file A");
    let file_b_contents = fs::read_to_string(b).expect("Unable to read file B");

    // Parse the JSON contents of file A and file B into Value objects
    let mut file_a_json: Value =
        serde_json::from_str(&file_a_contents).expect("Unable to parse JSON in file A");
    let file_b_json: Value =
        serde_json::from_str(&file_b_contents).expect("Unable to parse JSON in file B");

    // Convert the JSON contents of file B into a Vec<Value>
    let file_b_vec: Vec<Value> = serde_json::from_value(file_b_json.clone())
        .expect("Unable to convert JSON from file B to Vec");

    // Check if file A is an array and iterate over its objects
    if let Value::Array(a_array) = &mut file_a_json {
        for a_obj in a_array {
            // Find the value of the key field in the current object in file A
            let a_key = a_obj.get(&key).expect("Object in file A is missing key");

            // Find all objects in file B that have the same value for the key field
            let matching_objs: Vec<&Value> = file_b_vec
                .iter()
                .filter(|b| match (a_key.clone(), b.get(&key).cloned()) {
                    (Value::String(a_key_str), Some(Value::String(b_key_str))) => {
                        a_key_str == b_key_str
                    }
                    (Value::Number(a_key_num), Some(Value::Number(b_key_num))) => {
                        a_key_num == b_key_num
                    }
                    _ => false,
                })
                .collect();

            // If the current object in file A does not have an array called "matches", create one
            if let Value::Object(a_obj_map) = a_obj {
                if !a_obj_map.contains_key("matches") {
                    a_obj_map.insert("matches".to_string(), Value::Array(Vec::new()));
                }
            }

            // Add all matching objects from file B to the "matches" array in the current object in file A
            if let Value::Object(a_obj_map) = a_obj {
                if let Value::Array(matches_array) = a_obj_map.get_mut("matches").unwrap() {
                    for b_obj in matching_objs {
                        matches_array.push(b_obj.clone());
                    }
                }
            }
        }
    } else {
        panic!("File A JSON is not an array");
    }

    // Create a new JSON object with the combined contents of file A and file B
    let combined_json = json!({
        "combined": file_a_json,
    });

    // Write the new JSON object to a file called "combined.json"
    let combined_json_str = serde_json::to_string_pretty(&combined_json)
        .expect("Unable to convert combined JSON to string");
    fs::write("combined.json", combined_json_str).expect("Unable to write combined JSON to file");
}
