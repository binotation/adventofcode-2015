use serde_json::{Map, Value};

/// Just sum every number in the input, no deserialization required.
#[allow(dead_code)]
fn calculate_sum(numbers_json: &str) -> i64 {
    let mut sum = 0;
    let bytes = numbers_json.as_bytes();

    // Search for numbers
    let mut i = 0;
    while i < bytes.len() {
        // Found number
        if bytes[i].is_ascii_digit() {
            // Check if number is negative
            let signed = bytes[i - 1] == 45;

            // Search for end of number
            let mut number_end_index = i + 1;
            while bytes[number_end_index].is_ascii_digit() {
                number_end_index += 1;
            }

            // Parse and add to sum
            let n = numbers_json[i..number_end_index].parse::<i64>().unwrap();
            sum += if signed { -n } else { n };

            // Update loop index
            i = number_end_index;
        } else {
            i += 1;
        }
    }
    sum
}

/// Recursively calculate sum on json object
#[allow(dead_code)]
fn calculate_sum_ignore_red_object(numbers_map: &Map<String, Value>) -> i64 {
    let mut sum = 0;

    for k in numbers_map.keys() {
        match numbers_map.get(k).unwrap() {
            Value::Array(array) => sum += calculate_sum_ignore_red_array(array),

            Value::Number(num) => sum += num.as_i64().unwrap(),

            Value::Object(obj) => sum += calculate_sum_ignore_red_object(obj),

            Value::String(str) if str == "red" => return 0,
            _ => (),
        }
    }
    sum
}

/// Recursively calculate sum on json array
#[allow(dead_code)]
fn calculate_sum_ignore_red_array(numbers_array: &Vec<Value>) -> i64 {
    let mut sum = 0;

    for v in numbers_array {
        match v {
            Value::Array(array) => sum += calculate_sum_ignore_red_array(array),

            Value::Number(number) => sum += number.as_i64().unwrap(),

            Value::Object(obj) => sum += calculate_sum_ignore_red_object(obj),
            _ => (),
        }
    }
    sum
}

#[cfg(test)]
mod solution {
    use super::*;
    use crate::input::get_input::get_input;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn calclulate_sum() {
        let numbers = get_input("numbers_json").unwrap();
        assert_eq!(super::calculate_sum(&numbers), 119433);
    }

    #[test]
    fn calculate_sum_ignore_red() {
        let file = File::open("src/input/numbers_json.txt").unwrap();
        let reader = BufReader::new(file);
        let json: Value = serde_json::from_reader::<BufReader<File>, Value>(reader).unwrap();
        let numbers_obj = json.as_object().unwrap();
        assert_eq!(super::calculate_sum_ignore_red_object(numbers_obj), 68466);
    }
}
