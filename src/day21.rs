#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

#[allow(dead_code)]
const WEAPONS: [Item; 5] = [
    Item {
        cost: 8,
        damage: 4,
        armor: 0,
    },
    Item {
        cost: 10,
        damage: 5,
        armor: 0,
    },
    Item {
        cost: 25,
        damage: 6,
        armor: 0,
    },
    Item {
        cost: 40,
        damage: 7,
        armor: 0,
    },
    Item {
        cost: 74,
        damage: 8,
        armor: 0,
    },
];
#[allow(dead_code)]
const ARMOR: [Option<Item>; 6] = [
    None,
    Some(Item {
        cost: 13,
        damage: 0,
        armor: 1,
    }),
    Some(Item {
        cost: 31,
        damage: 0,
        armor: 2,
    }),
    Some(Item {
        cost: 53,
        damage: 0,
        armor: 3,
    }),
    Some(Item {
        cost: 75,
        damage: 0,
        armor: 4,
    }),
    Some(Item {
        cost: 102,
        damage: 0,
        armor: 5,
    }),
];
#[allow(dead_code)]
const RINGS: [Option<Item>; 7] = [
    None,
    Some(Item {
        cost: 25,
        damage: 1,
        armor: 0,
    }),
    Some(Item {
        cost: 50,
        damage: 2,
        armor: 0,
    }),
    Some(Item {
        cost: 100,
        damage: 3,
        armor: 0,
    }),
    Some(Item {
        cost: 20,
        damage: 0,
        armor: 1,
    }),
    Some(Item {
        cost: 40,
        damage: 0,
        armor: 2,
    }),
    Some(Item {
        cost: 80,
        damage: 0,
        armor: 3,
    }),
];

#[allow(dead_code)]
struct Boss {
    hitpoints: i32,
    damage: i32,
    armor: i32,
}

#[allow(dead_code)]
impl Boss {
    fn new() -> Self {
        Self {
            hitpoints: 100,
            damage: 8,
            armor: 2,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Player<'a> {
    hitpoints: i32,
    damage: i32,
    armor: i32,
    equipped_weapon: &'a Item,
    equipped_armor: Option<&'a Item>,
    equipped_left_ring: Option<&'a Item>,
    equipped_right_ring: Option<&'a Item>,
}

#[allow(dead_code)]
impl<'a> Player<'a> {
    fn new(
        weapon: &'a Item,
        armor: Option<&'a Item>,
        left_ring: Option<&'a Item>,
        right_ring: Option<&'a Item>,
    ) -> Self {
        let mut player = Self {
            hitpoints: 100,
            damage: 0,
            armor: 0,
            equipped_weapon: weapon,
            equipped_armor: armor,
            equipped_left_ring: left_ring,
            equipped_right_ring: right_ring,
        };
        player.update_damage();
        player.update_armor();
        player
    }

    fn defeats(&self, boss: Boss) -> bool {
        let player_adjusted_damage = if self.damage - boss.armor <= 0 {
            1
        } else {
            self.damage - boss.armor
        };
        let boss_adjusted_damage = if boss.damage - self.armor <= 0 {
            1
        } else {
            boss.damage - self.armor
        };

        let boss_is_defeated_in = boss.hitpoints / player_adjusted_damage;
        let player_is_deafeated_in = self.hitpoints / boss_adjusted_damage;
        // Player goes first so if number of attacks required are equal then player wins
        boss_is_defeated_in <= player_is_deafeated_in
    }

    fn update_damage(&mut self) {
        self.damage = self.equipped_weapon.damage;
        if let Some(left_ring) = &self.equipped_left_ring {
            self.damage += left_ring.damage;
        }
        if let Some(right_ring) = &self.equipped_right_ring {
            self.damage += right_ring.damage;
        }
    }

    fn update_armor(&mut self) {
        if let Some(equipped_armor) = &self.equipped_armor {
            self.armor = equipped_armor.armor;
        } else {
            self.armor = 0;
        }
        if let Some(left_ring) = &self.equipped_left_ring {
            self.armor += left_ring.armor;
        }
        if let Some(right_ring) = &self.equipped_right_ring {
            self.armor += right_ring.armor;
        }
    }

    fn get_cost_of_gear(&self) -> i32 {
        let mut cost = self.equipped_weapon.cost;
        if let Some(armor) = self.equipped_armor {
            cost += armor.cost
        }
        if let Some(ring) = self.equipped_left_ring {
            cost += ring.cost
        }
        if let Some(ring) = self.equipped_right_ring {
            cost += ring.cost
        }
        cost
    }
}

#[cfg(test)]
mod solution {
    use super::*;

    #[test]
    fn find_cheapest_gear() {
        let mut loadouts = Vec::new();
        // Brute force
        for weapon in WEAPONS.iter() {
            for armor in ARMOR.iter() {
                for right_ring in RINGS.iter() {
                    for left_ring in RINGS
                        .iter()
                        .filter(|&left_ring| (left_ring != right_ring) || *left_ring == None)
                    {
                        let player = Player::new(
                            weapon,
                            armor.as_ref(),
                            left_ring.as_ref(),
                            right_ring.as_ref(),
                        );

                        let boss = Boss::new();
                        // Keep loadouts where player wins
                        if player.defeats(boss) {
                            loadouts.push(player);
                        }
                    }
                }
            }
        }
        // Lowest cost loadout that wins
        assert_eq!(
            loadouts
                .into_iter()
                .min_by_key(|player| player.get_cost_of_gear())
                .unwrap()
                .get_cost_of_gear(),
            91
        );
    }

    #[test]
    fn find_most_expensive_gear() {
        let mut loadouts = Vec::new();
        // Brute force
        for weapon in WEAPONS.iter() {
            for armor in ARMOR.iter() {
                for right_ring in RINGS.iter() {
                    for left_ring in RINGS
                        .iter()
                        .filter(|&left_ring| (left_ring != right_ring) || *left_ring == None)
                    {
                        let player = Player::new(
                            weapon,
                            armor.as_ref(),
                            left_ring.as_ref(),
                            right_ring.as_ref(),
                        );

                        let boss = Boss::new();
                        // Keep loadouts where player loses
                        if !player.defeats(boss) {
                            loadouts.push(player);
                        }
                    }
                }
            }
        }
        // Highest cost loadout that loses
        assert_eq!(
            loadouts
                .into_iter()
                .max_by_key(|player| player.get_cost_of_gear())
                .unwrap()
                .get_cost_of_gear(),
            158
        );
    }

    #[test]
    fn battle_simple() {
        let player = Player {
            hitpoints: 8,
            damage: 5,
            armor: 5,
            equipped_weapon: &Item {
                damage: 5,
                armor: 5,
                cost: 0,
            },
            equipped_armor: None,
            equipped_left_ring: None,
            equipped_right_ring: None,
        };
        let mut boss = Boss::new();
        boss.hitpoints = 12;
        boss.damage = 7;
        boss.armor = 2;
        assert!(player.defeats(boss));
    }
}
