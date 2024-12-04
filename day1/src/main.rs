use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

fn main() {

    if let Ok(lines) = read_lines("locations.txt") {
        let lines_collection: Vec<_> = lines.flatten().collect();
        let line_count = lines_collection.len();

        let mut first_loc_array = Vec::with_capacity(line_count);
        let mut second_loc_array = Vec::with_capacity(line_count);

        let mut i = 0;  // index for arrays
        
        for line in lines_collection {
            let mut iter = line.split_whitespace();

            assert_eq!(iter.clone().count(), 2);

            first_loc_array.insert(i, iter.next().unwrap().parse::<i32>().unwrap());
            second_loc_array.insert(i, iter.next().unwrap().parse::<i32>().unwrap());

            i += 1;
        }

        first_loc_array.sort();
        second_loc_array.sort();

        let sum = sum_distance(&first_loc_array, &second_loc_array);
    
        println!("Sum is: {sum}");

        let part_2 = calculate_simularity_score(&first_loc_array, &second_loc_array);

        println!("Part 2: {part_2}");
    }
}

fn sum_distance(first_loc_array: &Vec<i32>, second_loc_array: &Vec<i32>) -> i32 {
    let mut sum = 0;

    assert_eq!(first_loc_array.len(), second_loc_array.len());

    for i in 0..first_loc_array.len() {
        sum += (first_loc_array[i] - second_loc_array[i]).abs();
    }
    sum
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calculate_simularity_score(first_loc_array: &Vec<i32>, second_loc_array: &Vec<i32>) -> i32 {
    let mut sum = 0;

    assert_eq!(first_loc_array.len(), second_loc_array.len());

    for i in 0..first_loc_array.len() {
        let indices: Vec<_> = second_loc_array
                                .iter()
                                .enumerate()
                                .filter_map(|(j, val)| if *val == first_loc_array[i] {Some(i)} else {None})
                                .collect();
        sum += first_loc_array[i] * i32::try_from(indices.len()).unwrap();
    }
    sum
}
