use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Entity {
    hit_points: i64,
    damage: i64,
    armor: i64,
    mana: i64,
}

#[derive(Debug, Clone)]
struct Spell {
    name: String,
    cost: i64,
    timer: i64,
    duration: i64,
}

fn get_my_stats() -> Entity {

    let e = Entity {
        hit_points: 50,
        damage: 0,
        armor: 0,
        mana: 500
    };

    return e;
}

fn boss_attack(boss: &mut Entity, player_spells: &mut HashMap<String, Spell>, player: &mut Entity) -> i64 {
    apply_effect(player, player_spells, boss);

    if got_winner(player,boss) == 1 {
        return 1;
    }

    let dmg = boss.damage - player.armor;
    let dmg = dmg.max(1);

    player.hit_points -= dmg;

    return 0;
}

fn apply_effect(player: &mut Entity, player_spells: &mut HashMap<String, Spell>, boss: &mut Entity) {
    for (name, spell) in player_spells {
        if spell.timer >= 1 {
            if *name == "Shield".to_string() {
                player.armor = 7;
            }
            if *name == "Poison".to_string() {
                boss.hit_points -= 3;
            }
            if *name == "Recharge".to_string() {
                player.mana += 101;
            }
            spell.timer -= 1;
        } else if *name == "Shield".to_string() && spell.timer == 0 {
            player.armor = 0;
        }
    } 
}

fn player_turn(player: &mut Entity, player_spells: &mut HashMap<String, Spell>, boss: &mut Entity) -> i64 {
    
    apply_effect(player, player_spells, boss);
    if got_winner(player, boss) == 1 {
        return 1;
    }

    return 0;
}

fn make_turns(player: &mut Entity, player_spells: &mut HashMap<String, Spell>, boss: &mut Entity, current_cost: i64, depth: i64, global_min: i64) -> i64 {

    // println!("{:?}, {:?}, {:?}, {}", player, player_spells, boss, current_cost);
    // println!("{}", depth);

    if current_cost >= global_min {
        return i64::MAX;
    }

    let mut player = player.clone();
    let mut boss = boss.clone();

    player.hit_points -= 1;
    match got_winner(&player, &boss) {
        2 => {
            return i64::MAX;
        },
        _ => {}
    } 

    if player_turn(&mut player, player_spells, &mut boss) == 1 {
        return current_cost;
    }

    let mut min = global_min;

    // make player turn
    let purchasable = get_purchasable_spells(player.clone(), player_spells.clone());
    if purchasable.is_empty() {
        return i64::MAX;
    } 

    for spell in purchasable {
        let mut player_clone = player.clone();
        let mut boss_clone = boss.clone();
        let mut player_spells_clone = player_spells.clone();
        let s = player_spells.get(&spell).unwrap();
        let new_min = current_cost + s.cost;
        player_clone.mana -= s.cost;

        // spell management ...
        if spell == "Magic Missile".to_string() {
            boss_clone.hit_points -= 4;
        } else if spell == "Drain".to_string() {
            boss_clone.hit_points -= 2;
            player_clone.hit_points += 2;
        } else if spell == "Shield".to_string() || spell == "Poison".to_string() || spell == "Recharge".to_string() {
            player_spells_clone.get_mut(&spell).unwrap().timer = player_spells_clone.get(&spell).unwrap().duration;
        }

        // TODO fix all this if things to be as the one after (because here we don't manage the boss win case)
        match got_winner(&player_clone, &boss_clone) {
            1 => { if new_min < min {
                        min = new_min;
                    }
                    continue;
                },
            2 => {
                continue;
            },
            _ => {}
        } 

        // boss turn
        let b = boss_attack(&mut boss_clone, &mut player_spells_clone, &mut player_clone);
        if b == 1 {
            if new_min < min {
                min = new_min;
            }
            continue;
        } else if b == 2 {
            continue;
        }

        match got_winner(&player_clone, &boss_clone) {
            1 => { if new_min < min {
                        min = new_min;
                    }
                    continue;
                },
            2 => {
                continue;
            },
            _ => {}
        } 

        // next round
        let new_min = make_turns(&mut player_clone, &mut player_spells_clone, &mut boss_clone, new_min, depth+1, min);
        if new_min < min {
            min = new_min;
        }
    }

    return min;

}

fn get_purchasable_spells(player: Entity, player_spells: HashMap<String, Spell>) -> Vec<String> {

    // obligation to cast a spell on each turn,
    // in the outer function treat the case where we return no spell to make the player lose 
    
    let mut purchasable = vec![];

    for (name, spell) in player_spells {
        if spell.timer == 0 && player.mana >= spell.cost {
            purchasable.push(name);
        }
    }

    purchasable

}

fn got_winner(a: &Entity, b: &Entity) -> i64 {

    if a.hit_points <= 0 {
        return 2;
    } else if b.hit_points <= 0 {
        return 1;
    }

    return 0;
}

fn parse_input(input: &Vec<String>) -> Entity {
    Entity {
        hit_points: input[0].split(": ").collect::<Vec<_>>()[1].parse::<i64>().unwrap(),
        damage: input[1].split(": ").collect::<Vec<_>>()[1].parse::<i64>().unwrap(),
        armor: 0,
        mana: 0,
    }
}


pub fn part2(input: &Vec<String>) -> i64 {
    let mut player_spells = HashMap::from([
        (String::from("Magic Missile"), Spell {name: String::from("Magic Missile"), cost: 53, timer: 0, duration: 0}),
        (String::from("Drain"), Spell {name: String::from("Drain"), cost: 73, timer: 0, duration: 0}),
        (String::from("Shield"), Spell {name: String::from("Shield"), cost: 113, timer: 0, duration: 6}),
        (String::from("Poison"), Spell {name: String::from("Poison"), cost: 173, timer: 0, duration: 6}),
        (String::from("Recharge"), Spell {name: String::from("Recharge"), cost: 229, timer: 0, duration: 5}),
    ]);

    let mut boss = parse_input(input);
    let mut player = get_my_stats();

    let res = make_turns(&mut player, &mut player_spells, &mut boss, 0, 0, i64::MAX);

    return res;
}