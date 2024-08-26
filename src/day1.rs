#[allow(dead_code)]
fn find_floor(directions: &str) -> i32 {
    let mut floor = 0;
    for c in directions.bytes() {
        match c {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => (),
        }
    }
    floor
}

#[allow(dead_code)]
fn find_basement_pos(directions: &str) -> Option<usize> {
    let mut floor = 0;
    for (i, c) in directions.bytes().enumerate() {
        match c {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            return Some(i + 1);
        }
    }
    None
}

#[cfg(test)]
mod solution {
    use super::*;
    use crate::input::get_input::get_input;

    #[test]
    fn find_floor_and_basement_pos() {
        let directions = get_input("directions").unwrap();
        let floor = find_floor(&directions);
        assert_eq!(floor, 232);

        let basement_pos = find_basement_pos(&directions).unwrap();
        assert_eq!(basement_pos, 1783);
    }
}
