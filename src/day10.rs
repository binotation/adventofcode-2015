#[allow(dead_code)]
const INPUT: &str = "1113122113";

#[allow(dead_code)]
fn look_and_say(sequence: &str) -> String {
    let mut output = String::new();
    let mut bytes = sequence.as_bytes().iter().peekable();
    while {
        let digit = bytes.next().unwrap();
        let mut count = 1;
        // Count digits
        while bytes.peek().is_some() && *bytes.peek().unwrap() == digit {
            bytes.next(); // Only consume if matches digit
            count += 1;
        }
        output.push_str(&format!("{}{}", count, *digit as char));
        bytes.peek().is_some()
    } {}
    output
}

#[cfg(test)]
mod solution {
    use super::*;

    #[test]
    fn test_look_and_say_simple() {
        let input = "111221";
        let output_sequence = look_and_say(&input);
        assert_eq!(output_sequence, "312211");
    }

    #[test]
    fn get_length_look_and_say40() {
        let mut output_sequence = look_and_say(INPUT);
        for _ in 0..39 {
            output_sequence = look_and_say(&output_sequence);
        }
        assert_eq!(output_sequence.len(), 360154);
    }

    #[test]
    fn get_length_look_and_say50() {
        let mut output_sequence = look_and_say(INPUT);
        for _ in 0..49 {
            output_sequence = look_and_say(&output_sequence);
        }
        assert_eq!(output_sequence.len(), 5103798);
    }
}
