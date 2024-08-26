//! Compile in release mode.
use rustc_hash::FxHashSet;

#[allow(dead_code)]
const MIN_PRESENTS: u32 = 29000000;

#[allow(dead_code)]
#[derive(Debug)]
struct House {
    num: u32,
    presents: u32,
}

#[allow(dead_code)]
fn get_factors(n: u32) -> FxHashSet<u32> {
    let mut factors = FxHashSet::default();
    for factor in 1..=f32::sqrt(n as f32) as u32 {
        if n % factor == 0 {
            factors.insert(factor);
            factors.insert(n / factor);
        }
    }
    factors
}

#[allow(dead_code)]
fn get_factors_not_infinite(n: u32) -> FxHashSet<u32> {
    let mut factors = FxHashSet::default();
    for factor in 1..=f32::sqrt(n as f32) as u32 {
        if n % factor == 0 {
            if n <= factor * 50 {
                factors.insert(factor);
            }
            if n <= (n / factor * 50) {
                factors.insert(n / factor);
            }
        }
    }
    factors
}

#[cfg(test)]
mod solution {
    use super::*;

    #[test]
    fn calculate_min_presents_house() {
        let mut house = House {
            num: 2,
            presents: 0,
        };
        while {
            house.presents = get_factors(house.num).iter().sum::<u32>() * 10;
            house.presents < MIN_PRESENTS
        } {
            house.num += 1;
        }
        assert_eq!(house.num, 665280);
    }

    #[test]
    fn calculate_min_presents_house_not_infinite() {
        let mut house = House {
            num: 2,
            presents: 0,
        };
        while {
            house.presents = get_factors_not_infinite(house.num).iter().sum::<u32>() * 11;
            house.presents < MIN_PRESENTS
        } {
            house.num += 1;
        }
        assert_eq!(house.num, 705600);
    }
}
