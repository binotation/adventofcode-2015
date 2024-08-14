/// This is the travelling salesman problem.
use itertools::Itertools;
use rustc_hash::FxHashMap;

/// Use the naive approach to the travelling salesman problem.
#[allow(dead_code)]
fn naive_tsp(happiness_matrix: &[Vec<i32>]) -> i32 {
    let guests: Vec<usize> = (0..happiness_matrix.len()).collect();

    let mut highest_happiness = 0;
    for seating in guests.into_iter().permutations(happiness_matrix.len()) {
        let mut happiness = 0;

        // Add happiness between guests
        for [guest1, guest2] in seating.array_windows::<2>() {
            happiness += happiness_matrix[*guest1][*guest2];
        }
        // Add happiness between last and first guest
        happiness += happiness_matrix[*seating.last().unwrap()][*seating.first().unwrap()];
        if happiness > highest_happiness {
            highest_happiness = happiness;
        }
    }
    highest_happiness
}

#[allow(dead_code)]
fn build_happiness_matrix(happiness_rules: &str) -> Vec<Vec<i32>> {
    let mut happiness_matrix: Vec<Vec<i32>> = Vec::new();
    // Asign each guest an index starting with 0
    let mut index = 0;
    let mut guests_index: FxHashMap<&str, usize> = FxHashMap::default();

    for line in happiness_rules.lines() {
        let line: Vec<&str> = line.split(" ").collect();
        let name1 = line[0];
        let name2 = &line[10][..line[10].len() - 1]; // Remove period "."
        let signed = line[2] == "lose";
        let happiness_change = {
            let happiness_change = line[3].parse::<i32>().unwrap();
            if signed {
                -happiness_change
            } else {
                happiness_change
            }
        };

        // Assign guest name an index if unassigned
        if !guests_index.contains_key(name1) {
            guests_index.insert(name1, index);
            // Add row into matrix, column is not needed because of the special ordering of the input text
            happiness_matrix.push(vec![0; index + 1]);
            index += 1;
        }
        if !guests_index.contains_key(name2) {
            guests_index.insert(name2, index);
            // Add row and column into matrix
            happiness_matrix.push(vec![0; index + 1]);
            for row in happiness_matrix.iter_mut().take(index) {
                row.push(0)
            }
            index += 1;
        }

        // Update happiness
        let name1_index = guests_index.get(name1).unwrap();
        let name2_index = guests_index.get(name2).unwrap();
        happiness_matrix[*name1_index][*name2_index] += happiness_change;
        happiness_matrix[*name2_index][*name1_index] += happiness_change;
    }
    happiness_matrix
}

#[cfg(test)]
mod solution {
    use super::*;
    use crate::input::get_input::get_input;

    #[test]
    fn get_highest_happiness() {
        let happiness = get_input("happiness").unwrap();
        let happiness_matrix = build_happiness_matrix(&happiness);
        assert_eq!(naive_tsp(&happiness_matrix), 733);
    }

    #[test]
    fn get_highest_happiness_including_me() {
        let happiness = get_input("happiness").unwrap();
        let mut happiness_matrix = build_happiness_matrix(&happiness);
        let n = happiness_matrix.len();
        happiness_matrix.push(vec![0; n + 1]);
        for row in happiness_matrix.iter_mut().take(n) {
            row.push(0);
        }
        assert_eq!(naive_tsp(&happiness_matrix), 725);
    }

    #[test]
    fn naive_tsp_simple() {
        let happiness_matrix = vec![
            vec![0, 54 + 83, -79 - 62, -2 + 46],
            vec![54 + 83, 0, -7 + 60, -63 - 7],
            vec![-79 - 62, 60 - 7, 0, 55 + 41],
            vec![-2 + 46, -63 - 7, 55 + 41, 0],
        ];
        assert_eq!(naive_tsp(&happiness_matrix), 330);
    }
}
