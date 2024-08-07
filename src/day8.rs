#[allow(dead_code)]
fn num_code_chars(characters: &str) -> usize {
    characters.len() - characters.lines().count() // Uncount newlines
}

#[allow(dead_code)]
fn num_memory_chars(characters: &str) -> usize {
    let mut count: usize = 0;
    for line in characters.lines() {
        let line_length = line.len();
        let mut line = line.as_bytes().iter();

        line.next(); // Throw away beginning double-quotation
        let mut i = 1;
        while {
            let c = line.next().unwrap();
            match c {
                b'\\' => {
                    match line.next().unwrap() {
                        b'\\' | b'"' => {
                            count += 1;
                            i += 2;
                        }
                        b'x' => {
                            // Throw away hex digits
                            line.next();
                            line.next();
                            count += 1;
                            i += 4;
                        }
                        _ => unreachable!(),
                    }
                }
                _ => {
                    count += 1;
                    i += 1;
                }
            }
            i < line_length
        } {}
        count -= 1; // Uncount ending double-quotation
    }
    count
}

#[allow(dead_code)]
fn num_encoded_chars(characters: &str) -> usize {
    let mut count: usize = 0;
    for line in characters.lines() {
        for c in line.bytes() {
            match c {
                b'\\' => count += 2,
                b'"' => count += 2,
                _ => count += 1,
            }
        }
        count += 2; // Count beginning and ending double-quotes
    }
    count
}

#[cfg(test)]
mod solution {
    use super::*;
    use crate::input::get_input::get_input;

    #[test]
    fn diff_num_code_num_memory_chars() {
        let characters = get_input("characters").unwrap();
        assert_eq!(
            num_code_chars(&characters) - num_memory_chars(&characters),
            1371
        );
    }

    #[test]
    fn diff_num_encoded_num_code_chars() {
        let characters = get_input("characters").unwrap();
        assert_eq!(
            num_encoded_chars(&characters) - num_code_chars(&characters),
            2117
        );
    }
}
