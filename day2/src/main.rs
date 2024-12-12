use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
    let file_name = "data.txt";
    if let Ok(lines) = read_lines(file_name) {
        let mut safe_map: HashMap<u32,bool> = HashMap::new();
        for (i, line) in lines.flatten().enumerate() {
            let numbers = line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let mut j: usize = 0;
            let mut is_safe: bool = true;
            let mut last_slope: i32 = 0;
            let mut droppable_count: u32 = 0;
            for value in numbers.iter() {
                if j < numbers.len() - 1 {
                    let difference:i32 = numbers[j+1] - value;
                    match &difference.cmp(&0) {
                        Ordering::Less => {
                            if difference.abs() > 3 {
                                if j == 0 {
                                    droppable_count += 1;
                                } else {
                                    if (numbers[j-1] - numbers[j+1]).abs() > 3 {
                                        if j+1 != numbers.len() - 1 && droppable_count == 0{
                                            is_safe = false;
                                        }
                                        droppable_count += 1;
                                    } else {
                                        droppable_count += 1;
                                    }
                                }
                            }
                            if j == numbers.len() - 2 && last_slope == 0 {
                                if (numbers[j-1] - numbers[j-2]).signum() != -1 {
                                    droppable_count += 1;
                                }
                            } else if j == numbers.len() - 2 && last_slope.signum() == 1 {
                                droppable_count += 1;
                            }
                            if j > 1 {
                                if last_slope.signum() == 1  {
                                    droppable_count += 1;
                                }
                            }
                        },
                        Ordering::Equal => {
                            droppable_count += 1;
                        },
                        Ordering::Greater => {
                            if difference.abs() > 3 {
                                if j == 0 {
                                    droppable_count += 1;
                                } else {
                                    if (numbers[j-1] - numbers[j+1]).abs() > 3 {
                                        if j+1 != numbers.len() - 1 && droppable_count == 0 {
                                            is_safe = false;
                                        }
                                        droppable_count += 1;
                                    } else {
                                        droppable_count += 1;
                                    }
                                }
                            }
                            // If the last slope was 0, then we need to check the previous slope
                            if j == numbers.len() - 2 && last_slope == 0{
                                if (numbers[j-1] - numbers[j-2]).signum() != 1 {
                                    droppable_count += 1;
                                }
                            }else if j == numbers.len() - 2 && last_slope.signum() == -1 {
                                droppable_count += 1;
                            }
                            if j > 1 {
                                if last_slope.signum() == -1 {
                                    droppable_count += 1;
                                }
                            }
                        }
                    }
                    println!("j: {} value: {} next: {} difference: {} slope: {} {}", j, value, numbers[j+1], difference, last_slope, droppable_count);
                    j += 1;
                    last_slope = difference;

                }
            }   
            if is_safe && droppable_count < 2 { 
                safe_map.insert(i as u32, true);
            } else {
                safe_map.insert(i as u32, false);
            }
        }
        for (k,v) in safe_map.iter() {
            println!("{}: {}", k+1, v);
        }
        if file_name != "data.txt" {
            verify(&safe_map);
        }

        let total = safe_map.values().filter(|&&value| value).count() as u32;
        println!("Safe map count: {}", total);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn verify(hashmap: &HashMap<u32,bool>) {
    for (k,v) in hashmap.iter() {
        println!("{}: {}", k, v);
    }
    assert_eq!(*hashmap.get(&0).unwrap(), true);
    assert_eq!(*hashmap.get(&1).unwrap(), false);
    assert_eq!(*hashmap.get(&2).unwrap(), false);
    assert_eq!(*hashmap.get(&3).unwrap(), true);
    assert_eq!(*hashmap.get(&4).unwrap(), true);
    assert_eq!(*hashmap.get(&5).unwrap(), true);
    assert_eq!(*hashmap.get(&6).unwrap(), true);
    assert_eq!(*hashmap.get(&7).unwrap(), false);
    assert_eq!(*hashmap.get(&8).unwrap(), false);
    assert_eq!(*hashmap.get(&9).unwrap(), false);
    assert_eq!(*hashmap.get(&10).unwrap(), false);
    assert_eq!(*hashmap.get(&11).unwrap(), true);
    assert_eq!(*hashmap.get(&12).unwrap(), false);
    assert_eq!(*hashmap.get(&13).unwrap(), false);
    assert_eq!(*hashmap.get(&14).unwrap(), false);
    assert_eq!(*hashmap.get(&15).unwrap(), false);
    assert_eq!(*hashmap.get(&16).unwrap(), false);
    assert_eq!(*hashmap.get(&17).unwrap(), false);
    assert_eq!(*hashmap.get(&18).unwrap(), false);
    assert_eq!(*hashmap.get(&19).unwrap(), false);
    assert_eq!(*hashmap.get(&20).unwrap(), false);
    assert_eq!(*hashmap.get(&21).unwrap(), false);
    assert_eq!(*hashmap.get(&22).unwrap(), false);
    assert_eq!(*hashmap.get(&23).unwrap(), false);
    assert_eq!(*hashmap.get(&24).unwrap(), false);
    assert_eq!(*hashmap.get(&25).unwrap(), false);
    assert_eq!(*hashmap.get(&26).unwrap(), false);
    assert_eq!(*hashmap.get(&27).unwrap(), false);
    assert_eq!(*hashmap.get(&28).unwrap(), true);
    assert_eq!(*hashmap.get(&29).unwrap(), false);
    assert_eq!(*hashmap.get(&30).unwrap(), false);
    assert_eq!(*hashmap.get(&31).unwrap(), false);
    assert_eq!(*hashmap.get(&32).unwrap(), true);
    assert_eq!(*hashmap.get(&33).unwrap(), false);
    assert_eq!(*hashmap.get(&34).unwrap(), false);
    assert_eq!(*hashmap.get(&35).unwrap(), false);
    assert_eq!(*hashmap.get(&36).unwrap(), false);
    assert_eq!(*hashmap.get(&37).unwrap(), false);
    assert_eq!(*hashmap.get(&38).unwrap(), false);
    assert_eq!(*hashmap.get(&39).unwrap(), false);
    assert_eq!(*hashmap.get(&40).unwrap(), false);
    assert_eq!(*hashmap.get(&41).unwrap(), false);
    assert_eq!(*hashmap.get(&42).unwrap(), false);



}
