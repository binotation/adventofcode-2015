use itertools::Itertools;

#[allow(dead_code)]
const PACKAGES_SIMPLE: [i64; 10] = [11, 10, 9, 8, 7, 5, 4, 3, 2, 1];
#[allow(dead_code)]
const GROUP_WEIGHT_SIMPLE: i64 = 20;

#[allow(dead_code)]
fn get_group_set(packages: &[i64], group_weight: i64, group_size: usize) -> Vec<Vec<i64>> {
    packages
        .iter()
        .copied()
        .combinations(group_size)
        .filter(|group| group.iter().sum::<i64>() == group_weight)
        .collect()
}

#[cfg(test)]
mod solution {
    use super::*;
    use crate::input::get_input::get_input;

    #[test]
    fn balance_sleigh_simple() {
        let min_group_size = 2;
        let group_set = get_group_set(&PACKAGES_SIMPLE, GROUP_WEIGHT_SIMPLE, min_group_size);

        let smallest_group_len = group_set.iter().min_by_key(|g| g.len()).unwrap().len();
        assert_eq!(smallest_group_len, min_group_size);

        // Get lowest quantum entanglement
        assert_eq!(
            group_set
                .into_iter()
                .filter(|g| g.len() == smallest_group_len)
                .map(|g| g.iter().product::<i64>())
                .min()
                .unwrap(),
            99
        );
    }

    #[test]
    fn balance_sleigh() {
        let packages: Vec<i64> = get_input("packages")
            .unwrap()
            .lines()
            .map(|p| p.parse().unwrap())
            .collect();
        let total_weight: i64 = packages.iter().sum();
        let group_weight = total_weight / 3;
        let min_group_size = 6;

        let group_set = get_group_set(&packages, group_weight, min_group_size);

        let smallest_group_len = group_set.iter().min_by_key(|g| g.len()).unwrap().len();
        assert_eq!(smallest_group_len, min_group_size);

        // Get lowest quantum entanglement
        assert_eq!(
            group_set
                .into_iter()
                .filter(|g| g.len() == smallest_group_len)
                .map(|g| g.iter().product::<i64>())
                .min()
                .unwrap(),
            11266889531
        );
    }

    #[test]
    fn balance_sleigh_4_groups() {
        let packages: Vec<i64> = get_input("packages")
            .unwrap()
            .lines()
            .map(|p| p.parse().unwrap())
            .collect();
        let total_weight: i64 = packages.iter().sum();
        let group_weight = total_weight / 4;
        let min_group_size = 5;

        let group_set = get_group_set(&packages, group_weight, min_group_size);

        let smallest_group_len = group_set.iter().min_by_key(|g| g.len()).unwrap().len();
        assert_eq!(smallest_group_len, min_group_size);

        // Get lowest quantum entanglement
        assert_eq!(
            group_set
                .into_iter()
                .filter(|g| g.len() == smallest_group_len)
                .map(|g| g.iter().product::<i64>())
                .min()
                .unwrap(),
            77387711
        );
    }
}
