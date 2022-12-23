use std::io;

#[derive(Copy, Clone, Debug)]
enum Choice{
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Copy, Clone)]
enum Conclusion{
    Defeat = 0,
    Draw = 3,
    Victory = 6,
}

#[derive(Copy, Clone, Debug)]
struct Round{
    op_choice: Choice,
    my_choice: Choice,
}

fn parse_choice(letter: char) -> Option<Choice>{
    match letter{
        'A' => Some(Choice::Rock),
        'B' => Some(Choice::Paper),
        'C' => Some(Choice::Scissors),
        _______ => None,
    }
}

fn get_my_choice(letter: char, their: Choice) -> Option<Choice>{
    match letter{
        'X' => match their{
            Choice::Rock => Some(Choice::Scissors),
            Choice::Paper => Some(Choice::Rock),
            Choice::Scissors => Some(Choice::Paper),
        },
        'Y' => match their{
            Choice::Rock => Some(Choice::Rock),
            Choice::Paper => Some(Choice::Paper),
            Choice::Scissors => Some(Choice::Scissors),
        },
        'Z' => match their{
            Choice::Rock => Some(Choice::Paper),
            Choice::Paper => Some(Choice::Scissors),
            Choice::Scissors => Some(Choice::Rock),
        },
        ___ => None,
    }
}

fn parse_round(line: String) -> Option<Round>{
    let their = match line.chars().nth(0){
        Some(ch) => parse_choice(ch), 
        None => None,
    };

    if their.is_none(){
        return None;
    }

    let mine = match line.chars().nth(2){
        Some(ch) => get_my_choice(ch, their.unwrap()), 
        None => None,
    };

    if mine.is_none() {
        return None;
    }else{
        Some(Round{op_choice:their.unwrap() , my_choice:mine.unwrap()})
    }
}

fn conclude_round(round: Round) -> Conclusion{
    match round.op_choice{
        Choice::Rock =>{
            match round.my_choice{
                Choice::Rock => return Conclusion::Draw,
                Choice::Paper => return Conclusion::Victory,
                Choice::Scissors => return Conclusion::Defeat,
            }
        },
        Choice::Paper =>{
            match round.my_choice{
                Choice::Rock => return Conclusion::Defeat,
                Choice::Paper => return Conclusion::Draw,
                Choice::Scissors => return Conclusion::Victory,
            }
        },
        Choice::Scissors =>{
            match round.my_choice{
                Choice::Rock => return Conclusion::Victory,
                Choice::Paper => return Conclusion::Defeat,
                Choice::Scissors => return Conclusion::Draw,
            }
        },
    }
}

fn main() {
    let stdin = io::stdin();
    let mut score: usize = 0;

    loop{
        let mut line = String::new();
        let res = stdin.read_line(&mut line);
        println!("{}", line);
        let round = parse_round(line);

        if round.is_some(){
            score += conclude_round(round.unwrap()) as usize;
            score += round.unwrap().my_choice as usize;
        }

        match res{
            Ok(0) => break,
            Ok(..) => {
                println!("{:?} - {}", round, score);
            },
            Err(x) => println!("{}", x),
        }
    }

    println!("Your score is: {}", score);
}
