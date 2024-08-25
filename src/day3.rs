use rustc_hash::FxHashSet;

#[allow(dead_code)]
fn count_unique_houses_delivered(directions: &str) -> i64 {
    let mut current_pos: (i64, i64) = (0, 0);
    let mut delivered_houses = FxHashSet::default();
    delivered_houses.insert(current_pos);
    let mut count = 1;

    for c in directions.bytes() {
        match c {
            b'^' => current_pos.1 += 1,
            b'v' => current_pos.1 -= 1,
            b'>' => current_pos.0 += 1,
            b'<' => current_pos.0 -= 1,
            _ => (),
        }
        let not_visited = delivered_houses.insert(current_pos);
        if not_visited {
            count += 1;
        }
    }
    count
}

#[allow(dead_code)]
fn count_unique_houses_delivered_with_robo_santa(directions: &str) -> i64 {
    let mut current_pos: [(i64, i64); 2] = [(0, 0); 2];
    let mut delivered_houses = FxHashSet::default();
    delivered_houses.insert(current_pos[0]);
    let mut count = 1;

    for (i, c) in directions.bytes().enumerate() {
        match c {
            b'^' => current_pos[i % 2].1 += 1,
            b'v' => current_pos[i % 2].1 -= 1,
            b'>' => current_pos[i % 2].0 += 1,
            b'<' => current_pos[i % 2].0 -= 1,
            _ => (),
        }
        let not_visited = delivered_houses.insert(current_pos[i % 2]);
        if not_visited {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod solution {
    use crate::input::get_input::get_input;

    #[test]
    fn count_unique_houses_delivered() {
        let directions = get_input("house_directions").unwrap();
        let count = super::count_unique_houses_delivered(&directions);
        assert_eq!(count, 2081);
    }

    #[test]
    fn count_unique_houses_delivered_with_robo_santa() {
        let directions = get_input("house_directions").unwrap();
        let count = super::count_unique_houses_delivered_with_robo_santa(&directions);
        assert_eq!(count, 2341);
    }
}
