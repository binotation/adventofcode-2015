use rustc_hash::FxHashMap;

const TIME_LIMIT: u32 = 2503;

#[allow(dead_code)]
enum State {
    Sprinting,
    Resting,
}

#[allow(dead_code)]
struct Reindeer<'a> {
    _name: &'a str,
    sprint_speed: u32,
    sprint_time: u32,
    rest_time: u32,
    state: State,
    sprinting_time: u32,
    resting_time: u32,
    distance_travelled: u32,
    points: u32,
}

#[allow(dead_code)]
struct ReindeerRace<'a> {
    reindeers: Vec<Reindeer<'a>>,
}

#[allow(dead_code)]
impl<'a> Reindeer<'a> {
    fn new(name: &'a str, sprint_speed: u32, sprint_time: u32, rest_time: u32) -> Self {
        Self {
            _name: name,
            sprint_speed,
            sprint_time,
            rest_time,
            state: State::Sprinting,
            sprinting_time: 0,
            resting_time: 0,
            distance_travelled: 0,
            points: 0,
        }
    }

    fn step_second(&mut self) {
        match self.state {
            State::Sprinting => {
                self.sprinting_time = (self.sprinting_time + 1) % self.sprint_time;
                self.distance_travelled += self.sprint_speed;
                if self.sprinting_time == 0 {
                    self.state = State::Resting;
                }
            }
            State::Resting => {
                self.resting_time = (self.resting_time + 1) % self.rest_time;
                if self.resting_time == 0 {
                    self.state = State::Sprinting;
                }
            }
        }
    }

    fn score_point(&mut self) {
        self.points += 1;
    }
}

#[allow(dead_code)]
impl<'a> ReindeerRace<'a> {
    fn new(reindeers: &'a str) -> Self {
        let mut reindeer_race = Self {
            reindeers: Vec::new(),
        };
        for line in reindeers.lines() {
            let line: Vec<&str> = line.split(" ").collect();
            let name = line[0];
            let sprint_speed: u32 = line[3].parse().unwrap();
            let sprint_time: u32 = line[6].parse().unwrap();
            let rest_time: u32 = line[13].parse().unwrap();
            reindeer_race
                .reindeers
                .push(Reindeer::new(name, sprint_speed, sprint_time, rest_time));
        }
        reindeer_race
    }

    fn step_second(&mut self) {
        for reindeer in self.reindeers.iter_mut() {
            reindeer.step_second();
        }
        let furthest_distance_travelled = self
            .reindeers
            .iter()
            .max_by_key(|r| r.distance_travelled)
            .unwrap()
            .distance_travelled;
        for reindeer in self
            .reindeers
            .iter_mut()
            .filter(|r| r.distance_travelled == furthest_distance_travelled)
        {
            reindeer.score_point();
        }
    }
}

#[allow(dead_code)]
fn find_fastest_reindeer(reindeers: &str) -> (&str, u32) {
    let mut reindeer: FxHashMap<&str, u32> = FxHashMap::default();

    for line in reindeers.lines() {
        let line: Vec<&str> = line.split(" ").collect();
        let name = line[0];
        let sprint_speed: u32 = line[3].parse().unwrap();
        let sprint_time: u32 = line[6].parse().unwrap();
        let rest_time: u32 = line[13].parse().unwrap();

        let sprint_distance = sprint_time * sprint_speed;
        let sprint_rest_time = sprint_time + rest_time;
        let sprints = TIME_LIMIT / sprint_rest_time;
        let remainder = TIME_LIMIT % sprint_rest_time;

        if remainder <= sprint_time {
            reindeer.insert(name, sprints * sprint_distance + (remainder * sprint_speed));
        } else {
            reindeer.insert(name, (sprints + 1) * sprint_distance);
        }
    }
    reindeer.into_iter().max_by_key(|v| v.1).unwrap()
}

#[cfg(test)]
mod solution {
    use super::*;
    use crate::input::get_input::get_input;

    #[test]
    fn find_fastest_reindeer() {
        let reindeers = get_input("reindeers").unwrap();
        let fastest_reindeer = super::find_fastest_reindeer(&reindeers);
        assert_eq!(fastest_reindeer.1, 2655);
    }

    #[test]
    fn find_fastest_reindeer_scored() {
        let reindeers = get_input("reindeers").unwrap();
        let mut reindeer_race = ReindeerRace::new(&reindeers);
        for _ in 0..TIME_LIMIT {
            reindeer_race.step_second();
        }

        let fastest_reindeer = reindeer_race
            .reindeers
            .into_iter()
            .max_by_key(|r| r.points)
            .unwrap();
        assert_eq!(fastest_reindeer.points, 1059);
    }
}
