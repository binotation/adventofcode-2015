use md5::{Digest, Md5};

#[allow(dead_code)]
const SECRET: &str = "ckczppom";

#[allow(dead_code)]
fn find_first_hash_with_condition(secret: &str, condition: impl Fn(&[u8]) -> bool) -> u64 {
    let mut count: u64 = 0;
    let mut msg = String::from(secret);

    while {
        msg.push_str(&count.to_string());

        let mut hasher = Md5::new();
        hasher.update(msg.as_bytes());
        let hash = hasher.finalize();

        !condition(&hash)
    } {
        msg.truncate(secret.len());
        count += 1;
    }
    count
}

#[cfg(test)]
mod solution {
    use super::*;

    #[test]
    fn find_first_hash_with_five_zeroes() {
        let count =
            super::find_first_hash_with_condition(SECRET, |h| h[0] == 0 && h[1] == 0 && h[2] < 16);
        assert_eq!(count, 117946);
    }

    #[test]
    fn find_first_hash_with_six_zeroes() {
        let count =
            super::find_first_hash_with_condition(SECRET, |h| h[0] == 0 && h[1] == 0 && h[2] == 0);
        assert_eq!(count, 3938038);
    }
}
