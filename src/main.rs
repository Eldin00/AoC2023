use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let exercise_to_run: (u8, u8) = (2, 2);

    match exercise_to_run {
        (1, 1) => {
            d1_part1();
        }
        (1, 2) => {
            d1_part2();
        }
        (2, 1) => {
            d2_part1();
        }
        (2, 2) => {
            d2_part2();
        }
        _ => {}
    }
}

fn d1_part1() { //Simple, brute-force solution. May come back to refactor at a later time.
    let file = File::open("./src/d1_part1_data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;

    for line in reader.lines() {
        match line {
            Ok(s) => {
                let d: Vec<char> = s.chars().filter(|c| c.is_digit(10)).collect();
                sum += (d[0].to_string() + &d[d.len() - 1].to_string())
                    .parse::<u32>()
                    .unwrap();
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
    println!("{sum}");
}

fn d1_part2() { //Simple, brute-force solution. May come back to refactor at a later time.
    let file = File::open("./src/d1_part1_data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;

    for line in reader.lines() {
        let mut digits: Vec<u32> = vec![];
        let temp_line = line.unwrap().to_ascii_lowercase();
        for i in 0..temp_line.len() {
            let chars = &temp_line[i..];
            let d = if chars.starts_with("1") || chars.starts_with("one") {
                '1'
            } else if chars.starts_with("2") || chars.starts_with("two") {
                '2'
            } else if chars.starts_with("3") || chars.starts_with("three") {
                '3'
            } else if chars.starts_with("4") || chars.starts_with("four") {
                '4'
            } else if chars.starts_with("5") || chars.starts_with("five") {
                '5'
            } else if chars.starts_with("6") || chars.starts_with("six") {
                '6'
            } else if chars.starts_with("7") || chars.starts_with("seven") {
                '7'
            } else if chars.starts_with("8") || chars.starts_with("eight") {
                '8'
            } else if chars.starts_with("9") || chars.starts_with("nine") {
                '9'
            } else if chars.starts_with("0") || chars.starts_with("zero") {
                '0'
            } else {
                ' '
            };
            if d.is_digit(10) {
                digits.push(d.to_digit(10).unwrap())
            }
        }
        sum += (digits[0] * 10) + (digits[digits.len() - 1]);
     }
    println!("{sum}");
}

fn d2_part1() {
    let file = File::open("./src/d2_part1_data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut games: Vec<i32> = vec![];
    let mut linenum = 1;

    for line in reader.lines() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let uline = line.unwrap();
        let parsed_line: Vec<&str> = uline.as_str()[uline.find(":").unwrap_or(0)+1..].trim().split(";").collect();

        for group in parsed_line {
            let set: Vec<&str> = group.trim().split(",").collect();
            for c in set {
                let cube: Vec<&str> = c.trim().split(" ").collect();
                match cube[1] {
                    "red" => { red = red.max(cube[0].parse::<u32>().unwrap_or(0)); },
                    "green" => { green = green.max(cube[0].parse::<u32>().unwrap_or(0)); },
                    "blue" => { blue = blue.max(cube[0].parse::<u32>().unwrap_or(0)); },
                    _ => {},
                };
            }
        }
        if red <= 12 && green <= 13 && blue <= 14 {
            games.push(linenum);
        };
        linenum += 1
    }
    println!("{}", games.iter().sum::<i32>());
}

fn d2_part2() {
    let file = File::open("./src/d2_part1_data.txt").unwrap();
    let reader = BufReader::new(file);    

    let mut games: Vec<u32> = vec![];

    for line in reader.lines() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let uline = line.unwrap();
        let parsed_line: Vec<&str> = uline.as_str()[uline.find(":").unwrap_or(0)+1..].trim().split(";").collect();

        for group in parsed_line {
            let set: Vec<&str> = group.trim().split(",").collect();
            for c in set {
                let cube: Vec<&str> = c.trim().split(" ").collect();
                match cube[1] {
                    "red" => { red = red.max(cube[0].parse::<u32>().unwrap_or(0)); },
                    "green" => { green = green.max(cube[0].parse::<u32>().unwrap_or(0)); },
                    "blue" => { blue = blue.max(cube[0].parse::<u32>().unwrap_or(0)); },
                    _ => {},
                };
            }
        }
        games.push(red * green * blue);
    }
    println!("{}", games.iter().sum::<u32>());

}