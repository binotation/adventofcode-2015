use std::cmp::Ordering;

#[allow(dead_code)]
fn count_container_combinations(
    containers: &[i32],
    remaining_eggnog: i32,
    count: &mut i32,
    depth: &mut i32,
    min_depth_known: i32,
    min_depth: &mut i32,
    min_depth_count: &mut i32,
) {
    for (i, container) in containers.iter().enumerate() {
        match (remaining_eggnog - container).cmp(&0) {
            Ordering::Less => continue, // Container is too large, skip
            Ordering::Equal => {
                // Exact fit
                *min_depth = std::cmp::min(*depth, *min_depth);
                *count += 1;
                if *depth == min_depth_known {
                    *min_depth_count += 1;
                }
            }
            Ordering::Greater => {
                // There is remaining eggnog to fit, recurse on remaining containers
                *depth += 1;
                count_container_combinations(
                    &containers[(i + 1)..],
                    remaining_eggnog - container,
                    count,
                    depth,
                    min_depth_known,
                    min_depth,
                    min_depth_count,
                );
            }
        }
    }
    *depth -= 1;
}

#[cfg(test)]
mod solution {
    use crate::input::get_input::get_input;

    #[test]
    fn count_container_combinations_simple() {
        let containers: Vec<i32> = vec![20, 15, 10, 5, 5];
        let mut count = 0;
        let mut depth = 1;
        let min_depth_known = 2;
        let mut min_depth = i32::MAX;
        let mut min_depth_count = 0;
        super::count_container_combinations(
            &containers,
            25,
            &mut count,
            &mut depth,
            min_depth_known,
            &mut min_depth,
            &mut min_depth_count,
        );
        assert_eq!(count, 4);
        assert_eq!(min_depth, min_depth_known);
        assert_eq!(min_depth_count, 3);
    }

    #[test]
    fn count_container_combinations() {
        let containers = get_input("containers").unwrap();
        let containers: Vec<i32> = containers.lines().map(|v| v.parse().unwrap()).collect();
        let mut count = 0;
        let mut depth = 1;
        let min_depth_known = 4;
        let mut min_depth = i32::MAX;
        let mut min_depth_count = 0;
        super::count_container_combinations(
            &containers,
            150,
            &mut count,
            &mut depth,
            min_depth_known,
            &mut min_depth,
            &mut min_depth_count,
        );
        assert_eq!(count, 1304);
        assert_eq!(min_depth, min_depth_known);
        assert_eq!(min_depth_count, 18);
    }
}
