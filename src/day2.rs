use std::{num::ParseIntError, str::FromStr};

struct Range {
    start: usize,
    end: usize,
}
impl FromStr for Range {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let part = s.split_once("-").unwrap();

        return Ok(Range {
            start: part.0.parse()?,
            end: part.1.parse()?,
        });
    }
}
pub fn solve() {
    let ip = INPUT;
    //let ip = TEST_INPUT;

    let mut invalid_ids = Vec::new();
    for r in ip.split(",").map(|p| Range::from_str(p).unwrap()) {
        for i in r.start..r.end+1 {
            if !is_valid2(i) {
                println!("Invlid id: {i}");
                invalid_ids.push(i);
            }
        }
    }

    println!("Final: {}", invalid_ids.iter().sum::<usize>());
}
fn is_valid2(num: usize) -> bool {
    let num_str = num.to_string();

    if num_str.len() == 2 {
        let (ch1, ch2) = num_str.split_at(1);
        return !(ch1 == ch2);
    }
    if num_str.len() == 3 {
        let mut ch = num_str.chars();
        let ch1 = ch.next().unwrap();
        let ch2 = ch.next().unwrap();
        let ch3 = ch.next().unwrap();
        return !(ch1 == ch2 && ch2 == ch3);
    }
    for position in (0..1+num_str.len()/2).rev() {
        let (p1, _) = num_str.split_at(position);

        //println!("Checking for: {num}, p1: {p1} and pos: {position}");
        if p1 == "" {
            continue;
        }
        let r = num_str.split(p1).all(|p| p == "");
        if r == true {
            println!("Got invalid entry: {num}, with part size: {position} and substr: {p1}");
            return false;
        }
    }

    return true;
}

fn is_valid(num: usize) -> bool {
    let num_str = num.to_string();
    if num_str.len() %2 != 0 {
        return true;
    }

    let (p1,p2) = num_str.split_at(num_str.len()/2);

    return p1 != p2;
}


const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

const INPUT: &str = "851786270-851907437,27-47,577-1044,1184-1872,28214317-28368250,47766-78575,17432-28112,2341-4099,28969-45843,5800356-5971672,6461919174-6461988558,653055-686893,76-117,2626223278-2626301305,54503501-54572133,990997-1015607,710615-802603,829001-953096,529504-621892,8645-12202,3273269-3402555,446265-471330,232-392,179532-201093,233310-439308,95134183-95359858,3232278502-3232401602,25116215-25199250,5489-8293,96654-135484,2-17";
