use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::HashMap,
};

fn main() { 
    let exercise_to_run: (u8, u8) = (5,2);

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
        (3, 1) => {
            d3_part1();
        }
        (3, 2) => {
            d3_part2();
        }
        (4, 1) => {
            d4_part1();
        }
        (4, 2) => {
            d4_part2();
        }
        (5, 1) => {
            d5_part1();
        }
        (5, 2) => {
            d5_part2();
        }
        _ => {}
    }
}

//most of these are simple and/or brute force solutions. I may revisit and try to refactor them into something more elegant in the future.

fn d1_part1() { 
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

fn d1_part2() { 
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

#[derive(Clone, Debug)]
struct D3P1Line {
    numbers: Vec<(usize, String)>,
    symbols: Vec<usize>,
}

impl D3P1Line {
    fn new() -> D3P1Line {
        D3P1Line { numbers: vec![], symbols: vec![] }
    }

    fn parse_line(&mut self, l: &str) -> D3P1Line {
        let mut buf: String = String::from("");
        let mut index: usize = 0;
        let mut offset = 0;
        for c in l.chars() {
            match c {
                '.' => { 
                    if !buf.is_empty() {
                        self.numbers.push((index, buf.clone()));
                        buf.clear();
                    } 
                },
                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                    if buf.is_empty() {
                        index = offset as usize;
                    }
                    buf.push(c);
                },
                _ => {
                    if !buf.is_empty() {
                        self.numbers.push((index, buf.clone()));
                        buf.clear();
                    }
                    self.symbols.push(offset);
                }
            };
            offset += 1;
        }
        if !buf.is_empty() {
            self.numbers.push((index, buf.clone()));
            buf.clear();
        }
    
        self.clone()
    }
    
}

fn d3_part1() {
    let file = File::open("./src/d3_part1_data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;
    let mut prev_line_parsed: D3P1Line = D3P1Line::new();

    for line in reader.lines() {
        let l = line.unwrap();
        let line_parsed: D3P1Line = D3P1Line::new().parse_line(&l);
        if prev_line_parsed.numbers.len() > 0 && line_parsed.symbols.len() > 0 {
            for s in &line_parsed.symbols {
                for n in &prev_line_parsed.numbers.clone() {
                    if s + 1 >= n.0 && *s <= n.0 + n.1.len() {
                        sum += n.1.parse::<i32>().unwrap_or(0);
                    }
                }
            }
        }
        if line_parsed.numbers.len() > 0 {
            if prev_line_parsed.symbols.len() > 0 {
                for s in &prev_line_parsed.symbols {
                    for n in &line_parsed.numbers.clone() {
                        if s + 1 >= n.0 && *s <= n.0 + n.1.len() {
                            sum += n.1.parse::<i32>().unwrap_or(0);
                        }
                    }
                }
            }
            if line_parsed.symbols.len() > 0 {
                for s in &line_parsed.symbols {
                    for n in &line_parsed.numbers.clone() {
                        if s + 1 >= n.0 && *s <= n.0 + n.1.len() {
                            sum += n.1.parse::<i32>().unwrap_or(0);
                        }
                    }
                }
            }
        }
        prev_line_parsed = line_parsed.clone();
    }
    println!("{sum}");
    
}

#[derive(Clone, Debug)]
struct D3P2Line {
    numbers: Vec<(usize, String)>,
    symbols: Vec<usize>,
}

impl D3P2Line {
    fn new() -> D3P2Line {
        D3P2Line { numbers: vec![], symbols: vec![] }
    }

    fn parse_line(&mut self, l: &str) -> D3P2Line {
        let mut buf: String = String::from("");
        let mut index: usize = 0;
        let mut offset: usize = 0;
        for c in l.chars() {
            match c {
                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                    if buf.is_empty() {
                        index = offset as usize;
                    }
                    buf.push(c);
                },
                '*' => {
                    if !buf.is_empty() {
                        self.numbers.push((index, buf.clone()));
                        buf.clear();
                    }
                    self.symbols.push(offset);
                }
                _ => { 
                    if !buf.is_empty() {
                        self.numbers.push((index, buf.clone()));
                        buf.clear();
                    } 
                },                
            };
            offset += 1;
        }
        if !buf.is_empty() {
            self.numbers.push((index, buf.clone()));
            buf.clear();
        }
    
        self.clone()
    }
    
}

fn d3_part2() {
    let file = File::open("./src/d3_part1_data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;
    
    let lines: Vec<D3P2Line> = reader.lines().map(|line| {D3P2Line::new().parse_line(&line.unwrap())}).collect();
    for i in 1..lines.len() {
        for s in &lines[i].symbols {
            let mut buf: Vec<i32> = vec![];
            if i != 0 {
                for n in &lines[i-1].numbers {
                    if s + 1 >= n.0 && *s <= n.0 + n.1.len() {
                        buf.push(n.1.parse::<i32>().unwrap_or(0));                    
                    }
                }
            }
            if i < lines.len() {
                for n in &lines[i+1].numbers {
                    if s + 1 >= n.0 && *s <= n.0 + n.1.len() {
                        buf.push(n.1.parse::<i32>().unwrap_or(0));                    
                    }
                }
            }
            for n in &lines[i].numbers {
                if s + 1 >= n.0 && *s <= n.0 + n.1.len() {
                    buf.push(n.1.parse::<i32>().unwrap_or(0));                    
                }
            }
            if buf.len() == 2 {
                sum += buf[0] * buf[1];
            }
        }
    }

    println!("{sum}")
}

fn d4_part1() {
    let file = File::open("./src/d4_data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;

    for line in reader.lines() {
        let pline = line.unwrap();
        let split_line: Vec<&str> = pline[pline.find(":").unwrap_or(0)+1..].trim().split("|").collect();
        let winners: Vec<i32> = split_line[0].trim().split(" ").filter(|value| !value.is_empty()).map(|value| { value.trim().parse::<i32>().unwrap_or(0)}).collect();
        let numbers: Vec<i32> = split_line[1].trim().split(" ").filter(|value| !value.is_empty()).map(|value| { value.trim().parse::<i32>().unwrap_or(0)}).filter(|value| winners.contains(value)).collect();
        if numbers.len() > 0 {
            sum += 2_i32.pow(numbers.len() as u32 - 1);
        }
    }
    println!("{sum}");
}

fn d4_part2() {
    let file = File::open("./src/d4_data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut copies: HashMap<i32, i32> = HashMap::new();
    let mut card = 1;
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap_or(String::from(""))).collect();

    for line in &lines {
        copies.insert(card, copies.get(&card).unwrap_or(&0) + 1);
        let split_line: Vec<&str> = line[line.find(":").unwrap_or(0)+1..].trim().split("|").collect();
        let winners: Vec<i32> = split_line[0].trim().split(" ").filter(|value| !value.is_empty()).map(|value| { value.trim().parse::<i32>().unwrap_or(0)}).collect();
        let numbers: Vec<i32> = split_line[1].trim().split(" ").filter(|value| !value.is_empty()).map(|value| { value.trim().parse::<i32>().unwrap_or(0)}).filter(|value| winners.contains(value)).collect();
        if numbers.len() > 0 {
            for i in card + 1..(numbers.len() as i32 + card + 1) {
                if i <= lines.len() as i32 {
                    copies.insert(i, copies.get(&i).unwrap_or(&0) + copies.get(&card).unwrap_or(&0));
                } 
            }
        }
        card += 1;
    }
    println!("{}", copies.values().sum::<i32>());
}

#[derive(Debug)]
struct D5MapLine {
    range_begin: i64,
    range_end: i64,
    offset: i64
}

impl D5MapLine {
    fn parse(d: i64, s: i64, l: i64) -> D5MapLine {
        D5MapLine { range_begin: s, range_end: s + l -1, offset: d - s }
    }
}

#[derive(Debug)]
struct D5Maps {
    seed_to_soil: Vec<D5MapLine>,
    soil_to_fert: Vec<D5MapLine>,
    fert_to_water: Vec<D5MapLine>,
    water_to_light: Vec<D5MapLine>,
    light_to_temp: Vec<D5MapLine>,
    temp_to_humid: Vec<D5MapLine>,
    humid_to_loc: Vec<D5MapLine>,
}

impl D5Maps {
    fn new() -> D5Maps {
        D5Maps{                         
                seed_to_soil: vec![],   //0
                soil_to_fert: vec![],   //1
                fert_to_water: vec![],  //2
                water_to_light: vec![], //3
                light_to_temp: vec![],  //4
                temp_to_humid: vec![],  //5
                humid_to_loc: vec![],   //6
            }
    }
}

fn d5_part1() {
    let file = File::open("./src/d5_data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut maps = D5Maps::new();
    let mut seeds: Vec<i32> = vec![];
    let mut current_map = -1;
    let mut locations: Vec<i64> = vec![];

    for line in reader.lines() {
        let pline = line.unwrap_or(String::from(""));
        if pline.starts_with("seeds") {
            seeds = pline.split(" ").map(|value| value.parse::<i32>().unwrap_or(0)).collect::<Vec<i32>>()[1..].to_vec();
        } else if pline.starts_with("seed-to-") {
            current_map = 0;
        } else if pline.starts_with("soil-to-") {
            current_map = 1;
        }else if pline.starts_with("fertilizer-to-") {
            current_map = 2;
        }else if pline.starts_with("water-to-") {
            current_map = 3;
        }else if pline.starts_with("light-to-") {
            current_map = 4;
        }else if pline.starts_with("temperature-to-") {
            current_map = 5;
        }else if pline.starts_with("humidity-to-") {
            current_map = 6;
        }else if pline.clone().chars().next().unwrap_or(' ').is_digit(10) {
            let l: Vec<i64> = pline.split(" ").map(|value| value.parse::<i64>().unwrap_or(0)).collect();
            match current_map {
                0 => { maps.seed_to_soil.push(D5MapLine::parse(l[0], l[1], l[2]) ); },
                1 => { maps.soil_to_fert.push(D5MapLine::parse(l[0], l[1], l[2]) ); },
                2 => { maps.fert_to_water.push(D5MapLine::parse(l[0], l[1], l[2]) ); },
                3 => { maps.water_to_light.push(D5MapLine::parse(l[0], l[1], l[2]) ); },
                4 => { maps.light_to_temp.push(D5MapLine::parse(l[0], l[1], l[2]) ); },
                5 => { maps.temp_to_humid.push(D5MapLine::parse(l[0], l[1], l[2]) ); },
                6 => { maps.humid_to_loc.push(D5MapLine::parse(l[0], l[1], l[2]) ); },
                _ => {}            
            }
        }
    }
    for s in seeds {
        let mut value = s as i64;
        for m in &maps.seed_to_soil {
            if value >= m.range_begin && value <= m.range_end { value += m.offset; break; }
        }
        for m in &maps.soil_to_fert {
            if value >= m.range_begin && value <= m.range_end { value += m.offset; break; }
        }
        for m in &maps.fert_to_water {
            if value >= m.range_begin && value <= m.range_end { value += m.offset; break; }
        }
        for m in &maps.water_to_light {
            if value >= m.range_begin && value <= m.range_end { value += m.offset; break; }
        }
        for m in &maps.light_to_temp {
            if value >= m.range_begin && value <= m.range_end { value += m.offset; break; }
        }
        for m in &maps.temp_to_humid {
            if value >= m.range_begin && value <= m.range_end { value += m.offset; break; }
        }
        for m in &maps.humid_to_loc {
            if value >= m.range_begin && value <= m.range_end { value += m.offset; break; }
        }
        locations.push(value);
    }

    println!("{:?}", locations.iter().min().unwrap())

}

fn d5_part2() {
    let file = File::open("./src/d5_data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut maps = D5Maps::new();
    let mut seeds: Vec<i64> = vec![];
    let mut current_map = -1;

    for line in reader.lines() {
        let pline = line.unwrap_or(String::from(""));
        if pline.starts_with("seeds") {
            seeds = pline.split(" ").map(|value| value.parse::<i64>().unwrap_or(0)).filter(|v| v != &0).collect::<Vec<i64>>().to_vec();
        } else if pline.starts_with("seed-to-") {
            current_map = 0;
        } else if pline.starts_with("soil-to-") {
            current_map = 1;
        }else if pline.starts_with("fertilizer-to-") {
            current_map = 2;
        }else if pline.starts_with("water-to-") {
            current_map = 3;
        }else if pline.starts_with("light-to-") {
            current_map = 4;
        }else if pline.starts_with("temperature-to-") {
            current_map = 5;
        }else if pline.starts_with("humidity-to-") {
            current_map = 6;
        }else if pline.clone().chars().next().unwrap_or(' ').is_digit(10) {
            let l: Vec<i64> = pline.split(" ").map(|value| value.parse::<i64>().unwrap_or(0)).collect();
            match current_map {
                0 => { maps.seed_to_soil.push(D5MapLine::parse(l[0], l[1], l[2]) ); },
                1 => { maps.soil_to_fert.push(D5MapLine::parse(l[0], l[1], l[2]) ); },
                2 => { maps.fert_to_water.push(D5MapLine::parse(l[0], l[1], l[2]) ); },
                3 => { maps.water_to_light.push(D5MapLine::parse(l[0], l[1], l[2]) ); },
                4 => { maps.light_to_temp.push(D5MapLine::parse(l[0], l[1], l[2]) ); },
                5 => { maps.temp_to_humid.push(D5MapLine::parse(l[0], l[1], l[2]) ); },
                6 => { maps.humid_to_loc.push(D5MapLine::parse(l[0], l[1], l[2]) ); },
                _ => {}            
            }
        }
    }

    let mut i = 0;
    let mut min_loc: i64 = i64::max_value();
    while i < seeds.len() {
        for s in seeds[i]..seeds[i] + seeds[i+1] {
            let mut value = s as i64;
            for m in &maps.seed_to_soil {
                if value >= m.range_begin && value <= m.range_end { value += m.offset; break; }
            }
            for m in &maps.soil_to_fert {
                if value >= m.range_begin && value <= m.range_end { value += m.offset; break; }
            }
            for m in &maps.fert_to_water {
                if value >= m.range_begin && value <= m.range_end { value += m.offset; break; }
            }
            for m in &maps.water_to_light {
                if value >= m.range_begin && value <= m.range_end { value += m.offset; break; }
            }
            for m in &maps.light_to_temp {
                if value >= m.range_begin && value <= m.range_end { value += m.offset; break; }
            }
            for m in &maps.temp_to_humid {
                if value >= m.range_begin && value <= m.range_end { value += m.offset; break; }
            }
            for m in &maps.humid_to_loc {
                if value >= m.range_begin && value <= m.range_end { value += m.offset; break; }
            }
            min_loc = if value < min_loc { value} else {min_loc};
        }
        i += 2;
    }

    println!("{min_loc}")

}



