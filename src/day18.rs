use std::str::FromStr;

#[allow(dead_code)]
struct LightGrid {
    grid: [[bool; 10000]; 2],
}

impl FromStr for LightGrid {
    type Err = String;

    fn from_str(light_grid_str: &str) -> Result<Self, Self::Err> {
        let mut light_grid = [[false; 10000]; 2];
        let mut i = 0;
        for b in light_grid_str.as_bytes() {
            match b {
                b'.' => i += 1,
                b'#' => {
                    light_grid[0][i] = true;
                    i += 1;
                }
                _ => (),
            }
        }
        Ok(Self { grid: light_grid })
    }
}

#[allow(dead_code)]
impl LightGrid {
    const ROWS: usize = 100;

    #[inline(always)]
    fn get(&self, toggle: usize, i: usize, j: usize) -> bool {
        self.grid[toggle][i * Self::ROWS + j]
    }

    #[inline(always)]
    fn set(&mut self, toggle: usize, i: usize, j: usize, state: bool) {
        self.grid[toggle][i * Self::ROWS + j] = state;
    }

    fn get_neighbours(&self, i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
        let neighbours: [(usize, usize); 8] = [
            (i.wrapping_sub(1), j.wrapping_sub(1)),
            (i.wrapping_sub(1), j),
            (i.wrapping_sub(1), j.wrapping_add(1)),
            (i, j.wrapping_sub(1)),
            (i, j.wrapping_add(1)),
            (i.wrapping_add(1), j.wrapping_sub(1)),
            (i.wrapping_add(1), j),
            (i.wrapping_add(1), j.wrapping_add(1)),
        ];
        neighbours
            .into_iter()
            .filter(|pos| pos.0 < Self::ROWS && pos.1 < Self::ROWS)
    }

    fn step_100(&mut self) -> i32 {
        for step in 0..100 {
            for i in 0..Self::ROWS {
                for j in 0..Self::ROWS {
                    let current_grid = step % 2;
                    let next_grid = (step + 1) % 2;

                    // Count ON neighbouring lights
                    let count: i32 = self
                        .get_neighbours(i, j)
                        .map(|light| self.get(current_grid, light.0, light.1) as i32)
                        .sum();

                    // Apply rules
                    if self.get(current_grid, i, j) {
                        if count == 2 || count == 3 {
                            self.set(next_grid, i, j, true);
                        } else {
                            self.set(next_grid, i, j, false);
                        }
                    } else if count == 3 {
                        self.set(next_grid, i, j, true);
                    } else {
                        self.set(next_grid, i, j, false);
                    }
                }
            }
        }
        self.grid[0].map(|light| light as i32).into_iter().sum()
    }

    const CORNERS: [(usize, usize); 4] = [(0, 0), (0, 99), (99, 0), (99, 99)];
    fn step_100_corners_always_on(&mut self) -> i32 {
        for step in 0..100 {
            for i in 0..Self::ROWS {
                for j in 0..Self::ROWS {
                    // Skip corner
                    if Self::CORNERS.contains(&(i, j)) {
                        continue;
                    }
                    let current_grid = step % 2;
                    let next_grid = (step + 1) % 2;

                    // Count ON neighbouring lights
                    let count: i32 = self
                        .get_neighbours(i, j)
                        .map(|light| self.get(current_grid, light.0, light.1) as i32)
                        .sum();

                    // Apply rules
                    if self.get(current_grid, i, j) {
                        if count == 2 || count == 3 {
                            self.set(next_grid, i, j, true);
                        } else {
                            self.set(next_grid, i, j, false);
                        }
                    } else if count == 3 {
                        self.set(next_grid, i, j, true);
                    } else {
                        self.set(next_grid, i, j, false);
                    }
                }
            }
        }
        self.grid[0].map(|light| light as i32).into_iter().sum()
    }
}

#[cfg(test)]
mod solution {
    use super::*;
    use crate::input::get_input::get_input;

    #[test]
    fn game_of_life() {
        let light_grid = get_input("light_grid").unwrap();
        let mut light_grid = light_grid.parse::<LightGrid>().unwrap();
        assert_eq!(light_grid.step_100(), 1061);
    }

    #[test]
    fn game_of_life_corners_always_on() {
        let light_grid = get_input("light_grid").unwrap();
        let mut light_grid = light_grid.parse::<LightGrid>().unwrap();
        for grid in 0..2 {
            light_grid.set(grid, 0, 0, true);
            light_grid.set(grid, 0, 99, true);
            light_grid.set(grid, 99, 0, true);
            light_grid.set(grid, 99, 99, true);
        }
        assert_eq!(light_grid.step_100_corners_always_on(), 1006);
    }
}
