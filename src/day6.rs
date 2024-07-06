use std::str::FromStr;

#[allow(dead_code)]
#[derive(Debug)]
struct Range((usize, usize), (usize, usize));

#[allow(dead_code)]
#[derive(Debug)]
enum Instruction {
    TurnOn(Range),
    TurnOff(Range),
    Toggle(Range),
}

trait Light {}
impl Light for bool {}
impl Light for i32 {}

#[allow(dead_code)]
trait ExecuteInstruction {
    #[inline(always)]
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Toggle(range) => {
                self.toggle(range);
            }
            Instruction::TurnOn(range) => {
                self.turn_on(range);
            }
            Instruction::TurnOff(range) => {
                self.turn_off(range);
            }
        }
    }

    fn turn_on(&mut self, range: Range);
    fn turn_off(&mut self, range: Range);
    fn toggle(&mut self, range: Range);
    fn aggregate(&self) -> i32;
}

impl FromStr for Instruction {
    type Err = String;

    #[inline(always)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(' ');
        let instruction_type; // 1 = Toggle, 2 = TurnOn, 3 = TurnOff

        // Match first token
        match tokens.next().unwrap() {
            "toggle" => {
                instruction_type = 1;
            }
            "turn" => {
                // Match second token
                match tokens.next().unwrap() {
                    "on" => {
                        instruction_type = 2;
                    }
                    "off" => {
                        instruction_type = 3;
                    }
                    // should be unreachable
                    _ => {
                        instruction_type = -1;
                    }
                }
            }
            // should be unreachable
            _ => {
                instruction_type = -1;
            }
        }
        // Map range string of the form "123,123" to iterator of usize
        let mut corner1 = tokens
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap());
        tokens.next(); // Throw away "through"
        let mut corner2 = tokens
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap());
        match instruction_type {
            1 => Ok(Instruction::Toggle(Range(
                (corner1.next().unwrap(), corner1.next().unwrap()),
                (corner2.next().unwrap(), corner2.next().unwrap()),
            ))),
            2 => Ok(Instruction::TurnOn(Range(
                (corner1.next().unwrap(), corner1.next().unwrap()),
                (corner2.next().unwrap(), corner2.next().unwrap()),
            ))),
            3 => Ok(Instruction::TurnOff(Range(
                (corner1.next().unwrap(), corner1.next().unwrap()),
                (corner2.next().unwrap(), corner2.next().unwrap()),
            ))),
            // Technically unreachable
            _ => Err(format!(
                "The following instruction could not be parsed: `{}`",
                s
            )),
        }
    }
}

struct LightGrid<T>
where
    T: Light,
{
    grid: Box<[T; 1000000]>,
}

impl<T> LightGrid<T>
where
    T: Light + Copy,
{
    #[allow(dead_code)]
    fn new_zeroed() -> LightGrid<T> {
        LightGrid {
            // SAFETY: safe for Light
            grid: unsafe { Box::new_zeroed().assume_init() },
        }
    }

    fn mutate_each(&mut self, range: Range, op: impl Fn(T) -> T) {
        for i in range.0 .0..=range.1 .0 {
            for j in range.0 .1..=range.1 .1 {
                self.grid[i * 1000 + j] = op(self.grid[i * 1000 + j]);
            }
        }
    }
}

impl ExecuteInstruction for LightGrid<bool> {
    fn toggle(&mut self, range: Range) {
        self.mutate_each(range, |light| !light)
    }

    fn turn_on(&mut self, range: Range) {
        self.mutate_each(range, |_| true)
    }

    fn turn_off(&mut self, range: Range) {
        self.mutate_each(range, |_| false)
    }

    fn aggregate(&self) -> i32 {
        self.grid
            .iter()
            .fold(0, |count, light| if *light { count + 1 } else { count })
    }
}

impl ExecuteInstruction for LightGrid<i32> {
    fn toggle(&mut self, range: Range) {
        self.mutate_each(range, |light| light + 2)
    }

    fn turn_on(&mut self, range: Range) {
        self.mutate_each(range, |light| light + 1)
    }

    fn turn_off(&mut self, range: Range) {
        self.mutate_each(range, |light| if light > 0 { light - 1 } else { 0 });
    }

    fn aggregate(&self) -> i32 {
        self.grid.iter().sum()
    }
}

#[cfg(test)]
mod solution {
    use super::*;
    use crate::input::get_input::get_input;

    #[test]
    fn get_ideal_lighting_configuration1() {
        let mut grid: LightGrid<bool> = LightGrid::new_zeroed();
        for line in get_input("instructions").unwrap().lines() {
            let instruction = line.parse::<Instruction>().unwrap();
            grid.execute(instruction);
        }
        assert_eq!(grid.aggregate(), 543903);
    }

    #[test]
    fn get_ideal_lighting_configuration2() {
        let mut grid: LightGrid<i32> = LightGrid::new_zeroed();
        for line in get_input("instructions").unwrap().lines() {
            let instruction = line.parse::<Instruction>().unwrap();
            grid.execute(instruction);
        }
        assert_eq!(grid.aggregate(), 14687245);
    }

    #[test]
    fn test_lightgrid() {
        let mut grid: LightGrid<bool> = LightGrid::new_zeroed();
        grid.turn_on(Range((499, 499), (500, 500)));
        assert_eq!(grid.aggregate(), 4);
    }
}
