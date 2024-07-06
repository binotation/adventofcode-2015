const FORBIDDEN: [[u8; 2]; 4] = [[97, 98], [99, 100], [112, 113], [120, 121]];
const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];

use rustc_hash::FxHashMap;

#[allow(dead_code)]
#[inline(always)]
fn is_nice_string(s: &str) -> bool {
    let mut vowel_count = 0;
    let mut double_letter: bool = false;
    for chunk in s
        .as_bytes()
        .windows(2)
        // SAFETY: chunk length (2) is equal to window length (2)
        .map(|w| unsafe { w.as_chunks_unchecked::<2>()[0] })
    {
        if vowel_count < 3 && VOWELS.contains(&chunk[0]) {
            vowel_count += 1;
        }
        if !double_letter {
            double_letter = chunk[0] == chunk[1];
        }
        if FORBIDDEN.contains(&chunk) {
            return false;
        }
    }
    // Check last char is vowel
    if VOWELS.contains(&s.bytes().last().unwrap()) {
        vowel_count += 1;
    }
    vowel_count > 2 && double_letter
}

/// O(3N) where N is the length of s
#[allow(dead_code)]
#[inline(always)]
fn is_nice_string2(s: &str) -> bool {
    let mut repeat_letters = false;
    let mut found_pairs = false;
    let mut pairs: FxHashMap<&[u8; 2], usize> = FxHashMap::default();

    for chunk in s
        .as_bytes()
        .windows(3)
        // SAFETY: chunk length (3) is equal to window length (3)
        .map(|w| unsafe { w.as_chunks_unchecked::<3>()[0] })
    {
        if !repeat_letters {
            repeat_letters = chunk[0] == chunk[2];
            if found_pairs {
                break;
            }
        }
    }
    // Look for identical chunks
    // SAFETY: str length (16) is divisible by chunk length (2)
    for (mut i, chunk) in unsafe { s.as_bytes().as_chunks_unchecked::<2>().iter().enumerate() } {
        i *= 2;
        if pairs.contains_key(chunk) {
            found_pairs = true;
            break;
        } else {
            pairs.insert(chunk, i);
        }
    }
    // View chunks offset +1, look for identical chunks and ensure the difference in their index is >1.
    if !found_pairs {
        // SAFETY: str length (14) is divisible by chunk length (2)
        for (mut i, chunk) in unsafe {
            s.as_bytes()[1..15]
                .as_chunks_unchecked::<2>()
                .iter()
                .enumerate()
        } {
            i = i * 2 + 1;
            if let Some(prev_i) = pairs.get(chunk) {
                found_pairs = (i as i32 - (*prev_i as i32)).abs() > 1;
                if found_pairs {
                    break;
                }
            } else {
                pairs.insert(chunk, i);
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
        let mut count = 0;
        for line in strings.lines() {
            if is_nice_string(line) {
                count += 1;
            }
        }
        assert_eq!(count, 238);

        count = 0;
        for line in strings.lines() {
            if is_nice_string2(line) {
                count += 1;
            }
        }
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
