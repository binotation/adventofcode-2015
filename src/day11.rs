#[allow(dead_code)]
const CURRENT_PASSWORD: &str = "cqjxjnds";

const FORBIDDEN: [u8; 3] = [105, 108, 111];

const ALPHABET: [u8; 23] = [
    97, 98, 99, 100, 101, 102, 103, 104, 106, 107, 109, 110, 112, 113, 114, 115, 116, 117, 118,
    119, 120, 121, 122,
];

#[allow(dead_code)]
fn is_valid_password(password: &str) -> bool {
    // Check if first 2 chars are forbidden
    if FORBIDDEN.contains(&password.as_bytes()[0]) {
        return false;
    }
    if FORBIDDEN.contains(&password.as_bytes()[1]) {
        return false;
    }

    // Check for straight while also checking no forbidden chars
    let mut has_straight = false;
    for [a, b, c] in password.as_bytes().array_windows::<3>() {
        if FORBIDDEN.contains(c) {
            return false;
        }
        if !has_straight && (*c == a + 2) && (*b == a + 1) {
            has_straight = true;
            break;
        }
    }

    if !has_straight {
        return false;
    }
    // Check for two different pairs
    let mut first_pair: Option<(&u8, &u8)> = None;
    for [a, b] in password.as_bytes().array_windows::<2>() {
        if first_pair.is_none() && a == b {
            first_pair = Some((a, b))
        } else if first_pair.is_some() && (a == b) && (first_pair.unwrap() != (a, b)) {
            return true;
        }
    }
    false
}

#[allow(dead_code)]
fn next_password(current_password: &mut str) {
    let mut carry;
    // SAFETY: current_password is ASCII so it is safe
    for c in unsafe { current_password.as_bytes_mut() }.iter_mut().rev() {
        let last_char: u8 = match c {
            105 => 104,
            108 => 107,
            111 => 110,
            _ => *c,
        };
        let mut char_index = ALPHABET.binary_search(&last_char).unwrap();
        char_index = if char_index == ALPHABET.len() - 1 {
            carry = true;
            0
        } else {
            carry = false;
            char_index + 1
        };
        *c = ALPHABET[char_index];
        if !carry {
            break;
        }
    }
}

#[cfg(test)]
mod solution {
    use super::*;

    #[test]
    fn get_next_valid_password1() {
        let mut password = String::from(CURRENT_PASSWORD);
        while !is_valid_password(&password) {
            next_password(&mut password);
        }
        assert_eq!(password, "cqjxxyzz");
    }

    #[test]
    fn get_next_valid_password2() {
        let mut password = String::from("cqjxxyzz");
        while {
            next_password(&mut password);
            !is_valid_password(&password)
        } {}
        assert_eq!(password, "cqkaabcc");
    }

    #[test]
    fn test_get_next_password_z() {
        let mut password = String::from("cqjxjndz");
        next_password(&mut password);
        assert_eq!(password, "cqjxjnea");
    }

    #[test]
    fn test_get_next_password_i() {
        let mut password = String::from("cqjxjnhz");
        next_password(&mut password);
        assert_eq!(password, "cqjxjnja");
    }
}
