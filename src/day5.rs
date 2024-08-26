const FORBIDDEN: [[u8; 2]; 4] = [[97, 98], [99, 100], [112, 113], [120, 121]];
const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];

use rustc_hash::FxHashMap;

#[allow(dead_code)]
#[inline(always)]
fn is_nice_string(s: &str) -> bool {
    let mut vowel_count = 0;
    let mut double_letter: bool = false;
    for window in s.as_bytes().array_windows::<2>() {
        if vowel_count < 3 && VOWELS.contains(&window[0]) {
            vowel_count += 1;
        }
        if !double_letter {
            double_letter = window[0] == window[1];
        }
        if FORBIDDEN.contains(window) {
            return false;
        }
    }
    // Check if last byte is a vowel
    if VOWELS.contains(&s.bytes().last().unwrap()) {
        vowel_count += 1;
    }
    vowel_count > 2 && double_letter
}

#[allow(dead_code)]
#[inline(always)]
fn is_nice_string2(s: &str) -> bool {
    let mut repeat_letters = false;
    let mut found_pairs = false;
    // Record byte pairs and their index
    let mut pairs: FxHashMap<&[u8; 2], usize> = FxHashMap::default();

    for window in s.as_bytes().array_windows::<3>() {
        if !repeat_letters {
            repeat_letters = window[0] == window[2];
            if found_pairs {
                break;
            }
        }
    }
    // Look for identical pairs
    // SAFETY: str length (16) is divisible by chunk length (2)
    for (i, pair) in unsafe { s.as_bytes().as_chunks_unchecked::<2>().iter().enumerate() } {
        let i = i * 2;
        if pairs.contains_key(pair) {
            found_pairs = true;
            break;
        } else {
            pairs.insert(pair, i);
        }
    }
    // View chunks offset +1, look for identical pairs and ensure the difference in their index is >1.
    if !found_pairs {
        // SAFETY: str length (14) is divisible by chunk length (2)
        for (i, pair) in unsafe {
            s.as_bytes()[1..15]
                .as_chunks_unchecked::<2>()
                .iter()
                .enumerate()
        } {
            let i = i * 2 + 1;
            if let Some(prev_i) = pairs.get(pair) {
                // If pair was previously found and index difference is >1
                found_pairs = (i as i32 - (*prev_i as i32)).abs() > 1;
                if found_pairs {
                    break;
                }
            } else {
                pairs.insert(pair, i);
            }
        }
    }
    repeat_letters && found_pairs
}

#[cfg(test)]
mod solution {
    use super::*;
    use crate::input::get_input::get_input;

    #[test]
    fn count_nice_strings() {
        let strings = get_input("strings").unwrap();
        let count = strings.lines().fold(0, |count, line| {
            if is_nice_string(line) {
                count + 1
            } else {
                count
            }
        });
        assert_eq!(count, 238);

        let count = strings.lines().fold(0, |count, line| {
            if is_nice_string2(line) {
                count + 1
            } else {
                count
            }
        });
        assert_eq!(count, 69);
    }

    #[test]
    fn test_is_nice_string2_general() {
        // repeat letter (hvh) with identical pair (st) at even spacing
        let s: &str = "qjhvhtzxzqqjkmpb";
        assert!(is_nice_string2(s));

        // no repeat letter with identical pair (st)
        let s: &str = "uurcxstgmygtbstg";
        assert!(!is_nice_string2(s));

        // repeat letter (odo) with no identical pair
        let s: &str = "ieodomkazucvgmuy";
        assert!(!is_nice_string2(s));
    }

    #[test]
    fn test_is_nice_string2_edge() {
        // repeat letter (zxz) with identical pair (qj) at odd spacing
        let s: &str = "qjcqjhtzxzaqkmpb";
        assert!(is_nice_string2(s));

        // ensure triple char doesn't trigger identical pair
        let s: &str = "qjcaaahzxzaqkmpb";
        assert!(!is_nice_string2(s));
    }
}
