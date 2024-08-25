use rustc_hash::FxHashSet;
use std::cmp::Ordering;

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Boss {
    hitpoints: i32,
    damage: i32,
    poisoned: Option<i32>,
}

#[allow(dead_code)]
impl Boss {
    fn new() -> Self {
        Self {
            hitpoints: 55,
            damage: 8,
            poisoned: None,
        }
    }

    fn new_custom(hitpoints: i32, damage: i32) -> Self {
        Self {
            hitpoints,
            damage,
            poisoned: None,
        }
    }

    fn attack(&mut self, enemy: &mut Wizard) {
        let adjusted_damage = if self.damage - enemy.armor <= 0 {
            1
        } else {
            self.damage - enemy.armor
        };
        enemy.hitpoints -= adjusted_damage;
    }

    fn apply_effect(&mut self) {
        if let Some(poison_timer) = self.poisoned.as_mut() {
            self.hitpoints -= 3;
            *poison_timer -= 1;
            if *poison_timer == 0 {
                self.poisoned = None;
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Wizard {
    hitpoints: i32,
    armor: i32,
    mana: i32,
    shielded: Option<i32>,
    recharging: Option<i32>,
    possible_spells: FxHashSet<Spell>,
}

#[allow(dead_code)]
impl Wizard {
    fn new() -> Self {
        let mut wizard = Self {
            hitpoints: 50,
            armor: 0,
            mana: 500,
            shielded: None,
            recharging: None,
            possible_spells: FxHashSet::default(),
        };
        wizard.update_possible_spells(&Boss::new());
        wizard
    }

    fn new_custom(hitpoints: i32, mana: i32) -> Self {
        let mut wizard = Self {
            hitpoints,
            armor: 0,
            mana,
            shielded: None,
            recharging: None,
            possible_spells: FxHashSet::default(),
        };
        wizard.update_possible_spells(&Boss::new());
        wizard
    }

    fn magic_missile(&mut self, enemy: &mut Boss) {
        self.mana -= Spell::MagicMissile.get_mana();
        enemy.hitpoints -= 4;
    }

    fn drain(&mut self, enemy: &mut Boss) {
        self.mana -= Spell::Drain.get_mana();
        enemy.hitpoints -= 2;
        self.hitpoints += 2;
    }

    fn shield(&mut self) {
        self.mana -= Spell::Shield.get_mana();
        if self.shielded.is_some() {
            panic!("Can not shield with existing shield");
        } else {
            self.shielded = Some(6);
            self.armor = 7;
        }
    }

    fn poison(&mut self, enemy: &mut Boss) {
        self.mana -= Spell::Poison.get_mana();
        if enemy.poisoned.is_some() {
            panic!("Cannot poison with existing poison");
        } else {
            enemy.poisoned = Some(6);
        }
    }

    fn recharge(&mut self) {
        self.mana -= Spell::Recharge.get_mana();
        if self.recharging.is_some() {
            panic!("Can not recharge with existing recharging");
        } else {
            self.recharging = Some(5);
        }
    }

    fn apply_effect(&mut self) {
        if let Some(shield_timer) = self.shielded.as_mut() {
            *shield_timer -= 1;
            if *shield_timer == 0 {
                self.armor = 0;
                self.shielded = None;
            }
        }
        if let Some(recharge_timer) = self.recharging.as_mut() {
            self.mana += 101;
            *recharge_timer -= 1;
            if *recharge_timer == 0 {
                self.recharging = None;
            }
        }
    }

    fn update_possible_spells(&mut self, enemy: &Boss) {
        if self.mana >= Spell::MagicMissile.get_mana() {
            self.possible_spells.insert(Spell::MagicMissile);
        } else {
            self.possible_spells.remove(&Spell::MagicMissile);
        }
        if self.mana >= Spell::Drain.get_mana() {
            self.possible_spells.insert(Spell::Drain);
        } else {
            self.possible_spells.remove(&Spell::Drain);
        }
        if self.mana >= Spell::Shield.get_mana()
            && (self.shielded.is_none() || self.shielded == Some(1))
        {
            self.possible_spells.insert(Spell::Shield);
        } else {
            self.possible_spells.remove(&Spell::Shield);
        }
        if self.mana >= Spell::Poison.get_mana()
            && (enemy.poisoned.is_none() || enemy.poisoned == Some(1))
        {
            self.possible_spells.insert(Spell::Poison);
        } else {
            self.possible_spells.remove(&Spell::Poison);
        }
        if self.mana >= Spell::Recharge.get_mana()
            && (self.recharging.is_none() || self.recharging == Some(1))
        {
            self.possible_spells.insert(Spell::Recharge);
        } else {
            self.possible_spells.remove(&Spell::Recharge);
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn get_mana(&self) -> i32 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Battle {
    wizard: Wizard,
    boss: Boss,
    hard_mode: bool,
    mana_used: i32,
    spells_used: Vec<Spell>,
    /// Did the the wizard win
    outcome: Option<bool>,
}

struct EffectOngoingError();

#[allow(dead_code)]
impl Battle {
    fn new(wizard: Wizard, boss: Boss, hard_mode: bool) -> Self {
        Self {
            wizard,
            boss,
            hard_mode,
            mana_used: 0,
            spells_used: Vec::new(),
            outcome: None,
        }
    }

    /// Returns Some(true) if the wizard won, Some(false) if the boss won or None
    /// if neither has won.
    fn step(&mut self, spell: &Spell) -> Result<Option<bool>, EffectOngoingError> {
        // Check chosen spell is possible
        if !self.wizard.possible_spells.contains(spell) {
            return Err(EffectOngoingError());
        }

        // Wizard's turn
        if self.hard_mode {
            self.wizard.hitpoints -= 1;
            if self.wizard.hitpoints <= 0 {
                self.outcome = Some(false);
                return Ok(Some(false));
            }
        }
        self.wizard.apply_effect();
        self.boss.apply_effect();

        match spell {
            Spell::MagicMissile => self.wizard.magic_missile(&mut self.boss),
            Spell::Drain => self.wizard.drain(&mut self.boss),
            Spell::Shield => self.wizard.shield(),
            Spell::Poison => self.wizard.poison(&mut self.boss),
            Spell::Recharge => self.wizard.recharge(),
        }
        self.mana_used += spell.get_mana();
        self.spells_used.push(spell.clone());

        if self.boss.hitpoints <= 0 {
            self.outcome = Some(true);
            return Ok(Some(true));
        }

        // Boss' turn
        self.wizard.apply_effect();
        self.boss.apply_effect();
        if self.boss.hitpoints <= 0 {
            self.outcome = Some(true);
            return Ok(Some(true));
        }

        self.boss.attack(&mut self.wizard);
        if self.wizard.hitpoints <= 0 {
            self.outcome = Some(false);
            return Ok(Some(false));
        }

        self.wizard.update_possible_spells(&self.boss);

        Ok(None)
    }
}

impl PartialEq for Battle {
    fn eq(&self, other: &Self) -> bool {
        self.mana_used == other.mana_used
    }
}

impl Eq for Battle {}

impl PartialOrd for Battle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Battle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.mana_used.cmp(&other.mana_used)
    }
}

/// Try 10 iterations of brute force, subsequent iterations will have battles that are taking
/// too long hence using too much mana.
#[allow(dead_code)]
fn find_least_mana_used_battle(hard_mode: bool) -> Vec<Battle> {
    // Rotating queue pair
    let mut queue: [Vec<Battle>; 2] = [Vec::new(), Vec::new()];

    // Battles the wizard has won
    let mut completed: Vec<Battle> = Vec::new();

    let initial = Battle::new(Wizard::new(), Boss::new(), hard_mode);
    queue[0].push(initial);

    let mut i = 0;
    while {
        // Rotate queue
        let (current_queue, next_queue) = if i % 2 == 0 {
            let (current, next) = queue.split_at_mut(1);
            (current, next)
        } else {
            let (next, current) = queue.split_at_mut(1);
            (current, next)
        };

        // Try each spell on each battle in current queue
        for battle in current_queue[0].iter() {
            for spell in &battle.wizard.possible_spells {
                let mut new_battle = battle.clone();
                let outcome = new_battle.step(spell);
                if let Ok(outcome) = outcome {
                    if let Some(won) = outcome {
                        if won {
                            completed.push(new_battle);
                        }
                    } else {
                        // Battle is unfinished, put back on queue
                        next_queue[0].push(new_battle);
                    }
                } else {
                    // Invalid spell, skip
                }
            }
        }

        current_queue[0].clear();
        i += 1;
        i < 10
    } {}
    completed
}

#[cfg(test)]
mod solution {
    use super::*;

    #[test]
    fn find_least_mana_used_battle() {
        let completed = super::find_least_mana_used_battle(false);
        assert_eq!(completed.into_iter().min().unwrap().mana_used, 953);
    }

    #[test]
    fn find_least_mana_used_battle_hard() {
        let completed = super::find_least_mana_used_battle(true);
        assert_eq!(completed.into_iter().min().unwrap().mana_used, 1289);
    }

    #[test]
    fn battle_simple() {
        let wizard = Wizard::new_custom(10, 250);
        let boss = Boss::new_custom(14, 8);
        let mut battle = Battle::new(wizard, boss, false);
        for spell in [
            Spell::Recharge,
            Spell::Shield,
            Spell::Drain,
            Spell::Poison,
            Spell::MagicMissile,
        ] {
            let outcome = battle.step(&spell);
            assert!(outcome.is_ok());
        }
        assert_eq!(battle.outcome, Some(true));
    }
}
