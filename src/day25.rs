/// Convert grid coord to position index.
#[allow(dead_code)]
fn grid_to_index(row: usize, col: usize) -> usize {
    let mut index = 0;
    let bound = col + row - 1;

    for offset_col in 1..=bound {
        if offset_col <= col {
            // Every column to the left (inclusive), add column diff
            index += row + (col - offset_col);
        } else {
            // Every column to the right, subtract (column diff + 1)
            let diff = offset_col - col + 1;
            index += row - diff;
        }
    }
    index
}

#[cfg(test)]
mod solution {
    use super::*;

    #[test]
    fn calculate_weather_machine_code() {
        let index = grid_to_index(2947, 3029);

        // Initial code
        let mut code: u64 = 20151125;
        for _ in 1..index {
            // Apply arithmetic
            code = (code * 252533) % 33554393;
        }
        assert_eq!(code, 19980801);
    }
}
