use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Entity {
    hit_points: i64,
    damage: i64,
    armor: i64,
}

#[derive(Debug, Clone)]
struct Item {
    name: String,
    cost: i64,
    damage: i64,
    defense: i64,
}

fn parse_input(input: &Vec<String>) -> Entity {
    Entity {
        hit_points: input[0].split(": ").collect::<Vec<_>>()[1].parse::<i64>().unwrap(),
        damage: input[1].split(": ").collect::<Vec<_>>()[1].parse::<i64>().unwrap(),
        armor: input[2].split(": ").collect::<Vec<_>>()[1].parse::<i64>().unwrap(),
    }
}

fn get_all_combinations(
    weapons: &Vec<Item>,
    armors: &Vec<Item>,
    rings: &Vec<Item>,
) -> Vec<(Item, Item, Vec<Item>)> {

    let ring_combinations: Vec<Vec<Item>> = (0..=2)
        .flat_map(|k| rings.iter().cloned().combinations(k))
        .collect();

    let mut combinations = Vec::new();

    for weapon in weapons {
        for armor in armors {
            for rings in &ring_combinations {
                combinations.push((
                    weapon.clone(),
                    armor.clone(),
                    rings.clone(),
                ));
            }
        }
    }
    combinations
}

fn get_my_stats(combination: (Item, Item, Vec<Item>)) -> (Entity, i64) {

    let mut cost = combination.0.cost + combination.1.cost;
    let mut damage = combination.0.damage + combination.1.damage;
    let mut defense = combination.0.defense + combination.1.defense;

    for ring in combination.2.iter() {
        cost += ring.cost;
        damage += ring.damage;
        defense += ring.defense;
    }

    let e = Entity {
        hit_points: 100,
        damage,
        armor: defense
    };

    return (e, cost);
}

fn attack(a: &mut Entity, b: &mut Entity) {
    let dmg = a.damage - b.armor;
    let dmg = dmg.max(1);

    b.hit_points -= dmg;
}

fn got_winner(a: &Entity, b: &Entity) -> i64 {

    if a.hit_points <= 0 {
        return 2;
    } else if b.hit_points <= 0 {
        return 1;
    }

    return 0;
}

fn fight(a: &Entity, b: &Entity) -> bool {
    let mut player = a.clone();
    let mut boss = b.clone();
    let mut player_turn = true;

    while got_winner(&player, &boss) == 0 {
        if player_turn {
            attack(&mut player, &mut boss);
        } else {
            attack(&mut boss, &mut player);
        }
        player_turn = !player_turn;
    }

    return got_winner(&player, &boss) == 1;
}

pub fn part2(input: &Vec<String>) -> i64 {
    let weapons: Vec<Item> = vec![
        Item { name: "Dagger".to_string(), cost: 8, damage: 4, defense: 0 },
        Item { name: "Shortsword".to_string(), cost: 10, damage: 5, defense: 0 },
        Item { name: "Warhammer".to_string(), cost: 25, damage: 6, defense: 0 },
        Item { name: "Longsword".to_string(), cost: 40, damage: 7, defense: 0 },
        Item { name: "Greataxe".to_string(), cost: 74, damage: 8, defense: 0 },
    ];

    let armors: Vec<Item> = vec![
        Item { name: "No Armor".to_string(), cost: 0, damage: 0, defense: 0 },
        Item { name: "Leather".to_string(),    cost: 13, damage: 0, defense: 1 },
        Item { name: "Chainmail".to_string(),  cost: 31, damage: 0, defense: 2 },
        Item { name: "Splintmail".to_string(), cost: 53, damage: 0, defense: 3 },
        Item { name: "Bandedmail".to_string(), cost: 75, damage: 0, defense: 4 },
        Item { name: "Platemail".to_string(),  cost: 102, damage: 0, defense: 5 },
    ];

    let rings: Vec<Item> = vec![
        Item { name: "Damage +1".to_string(),  cost: 25,  damage: 1, defense: 0 },
        Item { name: "Damage +2".to_string(),  cost: 50,  damage: 2, defense: 0 },
        Item { name: "Damage +3".to_string(),  cost: 100, damage: 3, defense: 0 },
        Item { name: "Defense +1".to_string(), cost: 20,  damage: 0, defense: 1 },
        Item { name: "Defense +2".to_string(), cost: 40,  damage: 0, defense: 2 },
        Item { name: "Defense +3".to_string(), cost: 80,  damage: 0, defense: 3 },
    ];

    let boss = parse_input(input);
    // println!("{:?}", boss);

    let combinations = get_all_combinations(&weapons, &armors, &rings);

    let mut max = 0;

    for c in combinations {
        let (p, cost) = get_my_stats(c);
        if !fight(&p, &boss) {
            if cost > max {
                max = cost;
            }
        }
    }

    return max;
}