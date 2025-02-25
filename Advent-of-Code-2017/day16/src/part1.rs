enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char)
}

fn parse_input(input: &String) -> Vec<Move> {
    let mut moves = Vec::new();
    let input = input.split(",").collect::<Vec<_>>();

    for m in input.into_iter() {
        let (m, infos) = m.split_at(1);
        match m {
            "s" => {
                moves.push(Move::Spin(infos.parse::<usize>().unwrap()))
            },
            "x" => {
                let infos = infos.split_once("/").unwrap();
                moves.push(Move::Exchange(infos.0.parse::<usize>().unwrap(), infos.1.parse::<usize>().unwrap()))
            },
            "p" => {
                let infos = infos.split_once("/").unwrap();
                moves.push(Move::Partner(infos.0.chars().nth(0).unwrap(), infos.1.chars().nth(0).unwrap()))
            }
            _ => panic!("Unknown")
        }
    }
    moves
}

fn dance(dance_moves: &Vec<Move>, programs: &mut Vec<char>) {
    for mv in dance_moves.iter() {
        match mv {
            Move::Spin(i) => {
                programs.rotate_right(*i);
            },
            Move::Exchange(i, j) => {
                programs.swap(*i, *j);
            },
            Move::Partner(a, b) => {
                let i = programs.iter().position(|x| x == a).unwrap();
                let j = programs.iter().position(|x| x == b).unwrap();
                programs.swap(i, j);
            }
        }
    }
}

pub fn part1(input: &Vec<String>) -> String {

    let dance_moves = parse_input(&input[0]);

    let mut programs = Vec::new();
    for c in 97..(97 + 16) {
        programs.push(c as u8 as char);
    }

    dance(&dance_moves, &mut programs);

    return programs.iter().collect::<String>();
}