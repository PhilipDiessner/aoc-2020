use crate::util;

pub fn part1(filename: &str) {
    let input = util::filename_to_string(filename).unwrap();
    let passes = util::read_map(input.as_str());
    let mut ids: Vec<u32> = Vec::new();
    for pass in passes {
        let mut row = String::new();
        for fb in pass.to_vec() {
            match fb {
                'F' => row += "0",
                'B' => row += "1",
                'L' => row += "0",
                'R' => row += "1",
                _ => panic!()
            }
        }
        ids.push(u32::from_str_radix(row.as_str(), 2).unwrap());
    }
    ids.sort();
    let max = ids.iter().max().unwrap().clone();
    let mut start = ids[0];
    let mut spot = start;
    for id in ids {
        if start == id { start += 1 } else {
            spot = start;
            break;
        }
    }
    println!("{}, {}", max, spot);
}

pub fn part2(filename: &str) {}