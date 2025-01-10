use std::collections::HashMap;
use regex::Regex;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
struct Bot {
    id: i32,
    low: i32,
    high: i32,
    out_low: String,
    out_high: String,
}

impl Bot {
    fn new(id: &String) -> Bot {
        Bot {
            id: id.parse::<i32>().unwrap(),
            low: 0,
            high: 0,
            out_low: "".to_string(),
            out_high: "".to_string()
        }
    }

    fn has_two_chips(& self) -> bool {
        return self.low != 0 && self.high != 0
    }

    fn get_chips(&self) -> (i32, i32) {
        return (self.high, self.low);
    }

    fn receive_chip(&mut self, chip: i32) {
        assert!(
            !self.has_two_chips(),
            "Assertion failed: Bot '{}' does not have two chips. Current bot : {:?}, Trying to receive: {:?}",
            self.id,
            self,
            chip
        );

        let c = self.high;
        if c > chip {
            self.low = chip;
        } else {
            self.high = chip;
            self.low = c;
        }
    }

    fn give_chips(&mut self, bots: &mut HashMap<String,Arc<Mutex<Bot>>>, output: &mut Vec<i32>, history: &mut HashMap<(i32, i32), i32>) {
        assert!(self.has_two_chips());

        
        history.insert(self.get_chips(), self.id);
        
        if self.out_low.starts_with("o") {
            let i = self.out_low[1..].parse::<usize>().unwrap();
            output[i] = self.low;
        } else {
            let b = bots.get_mut(&self.out_low).expect(format!("Bot {} not found", self.out_low).as_str());
            b.lock().unwrap().receive_chip(self.low);
            self.low = 0;
        }


        if self.out_high.starts_with("o") {
            let i = self.out_high[1..].parse::<usize>().unwrap();
            output[i] = self.high;
        } else {
            let b = bots.get_mut(&self.out_high).unwrap();
            b.lock().unwrap().receive_chip(self.high);
            self.high = 0;
        }

    }

}

fn parse_input(input: &Vec<String>) -> HashMap<String, Arc<Mutex<Bot>>> {

    let mut bots = HashMap::new();
    let r1 = Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)").unwrap();
    let r2 = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();

    for line in input {

        if let Some(caps) = r1.captures(&line) {
            let id = caps.get(1).unwrap().as_str().to_string();
            let mut idl = caps.get(3).unwrap().as_str().to_string();
            let mut idh = caps.get(5).unwrap().as_str().to_string();

            let b1 = caps.get(2).unwrap().as_str().to_string();
            let b2 = caps.get(4).unwrap().as_str().to_string();

            if b1 == "output" {
                idl = String::from("o") + &idl;
            }
            if b2 == "output" {
                idh = String::from("o") + &idh;
            }

            let bot = bots.entry(id.clone()).or_insert(Arc::new(Mutex::new(Bot::new(&id))));
            let mut bot = bot.lock().unwrap();
            bot.out_low = idl;
            bot.out_high = idh;

        } else if let Some(caps) = r2.captures(&line) {
            let id = caps.get(2).unwrap().as_str().to_string();
            let val = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();

            let bot = bots.entry(id.clone()).or_insert(Arc::new(Mutex::new(Bot::new(&id))));
            let mut bot = bot.lock().unwrap();
            bot.receive_chip(val);
            
        } else {
            panic!("Unknown line pattern")
        }

    }

    bots

}

fn solve(bots: &mut HashMap<String,Arc<Mutex<Bot>>>, output: &mut Vec<i32>) -> HashMap<(i32, i32), i32> {

    let mut history = HashMap::new();

    while output.into_iter().any(|x| *x == 0) {

        let mut bots_clone = bots.clone();
        let current_bot = bots_clone.values_mut()
        .find(|bot| bot.lock().unwrap().has_two_chips())
        .expect("No bot with two chips found");

        current_bot.lock().unwrap().give_chips(bots, output, &mut history);

    }

    history
}

fn find_in_history((high, low): (i32, i32), history: &mut HashMap<(i32, i32), i32>) -> i32 {
    for ((h, l), id) in history.iter() {
        if *h == high && *l == low {
            return *id;
        }
    }
    return -1;
}

fn get_string_output(output: Vec<i32>) -> String {
    output.iter().map(|x| x.to_string()).collect::<String>()
}

pub fn part1(input: &Vec<String>) -> i64 {

    let mut output =  vec![0; 21];
    let mut bots = parse_input(input);

    let mut history = solve(&mut bots, &mut output);

    // println!("{}", get_string_output(output)); 

    return find_in_history((61, 17), &mut history) as i64;

}