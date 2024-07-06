#[allow(dead_code)]
/// Decompose a string of the form "lxwxh" to [i64; 3].
fn decompose_present_dimensions(present: &str) -> [i64; 3] {
    let mut dims = present.split('x');
    // We can make the assumption that there are always 3 numbers and they are valid ints.
    [
        dims.next().unwrap().parse().unwrap(),
        dims.next().unwrap().parse().unwrap(),
        dims.next().unwrap().parse().unwrap(),
    ]
}

#[allow(dead_code)]
fn calculate_required_wrapping_paper(presents: &str) -> i64 {
    let mut total: i64 = 0;
    for present in presents.lines() {
        let [l, w, h] = decompose_present_dimensions(present);
        let faces_area = [l * w, w * h, h * l];
        total = total + 2 * faces_area.iter().sum::<i64>() + faces_area.iter().min().unwrap();
    }
    total
}

#[allow(dead_code)]
fn calculate_required_ribbon(presents: &str) -> i64 {
    let mut total = 0;
    for present in presents.lines() {
        let dims = decompose_present_dimensions(present);
        let (longest_side, _) = dims.iter().enumerate().max_by_key(|(_, &v)| v).unwrap();
        // Add length of 2 shortest sides twice
        for (side, length) in dims.iter().enumerate() {
            if side == longest_side {
                continue;
            }
            total += 2 * *length;
        }
        total += dims.into_iter().product::<i64>();
    }
    total
}

#[cfg(test)]
mod solution {
    use super::*;
    use crate::input::get_input::get_input;

    #[test]
    fn decompose_test() {
        let s: &str = "24x25x17";
        let [l, w, h] = decompose_present_dimensions(s);
        assert_eq!(l, 24);
        assert_eq!(w, 25);
        assert_eq!(h, 17);
    }

    #[test]
    fn calculate_required_wrapping_paper() {
        let area = super::calculate_required_wrapping_paper(&get_input("presents").unwrap());
        assert_eq!(area, 1588178);
    }

    #[test]
    fn calculate_required_ribbon() {
        let length = super::calculate_required_ribbon(&get_input("presents").unwrap());
        assert_eq!(length, 3783758);
    }
}
