# Improved solution recommended by ChatGPT

Adding this for reference and to understand Rust better.

New solution improves:
```text
Improvements
Avoid Vec<i32> References for Function Parameters: Use slices (&[i32]) instead of &Vec<i32> for function parameters unless you need Vec-specific functionality.

Use Iterators Instead of Indexing Loops: Replace indexed loops (for i in 0..vec.len()) with iterator methods to make the code more idiomatic and concise.

Remove Unnecessary Assertions: Assertions, such as ensuring the iterator count matches 2, can be replaced with proper error handling for better robustness.

Optimize calculate_simularity_score: The current approach of filtering and collecting indices is inefficient. You can optimize it using more direct logic.

Replace Vec::insert with Vec::push: Since you're inserting elements sequentially, push is faster and more natural than insert.

Add Error Handling: Improve error handling to provide meaningful feedback when parsing fails.

General Cleanup: Simplify some code and fix formatting for better readability.
```


```rust
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("locations.txt") {
        let mut first_loc_array = Vec::new();
        let mut second_loc_array = Vec::new();

        for (line_number, line) in lines.flatten().enumerate() {
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.len() != 2 {
                eprintln!("Error: Invalid line format at line {}", line_number + 1);
                return;
            }

            match (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                (Ok(first), Ok(second)) => {
                    first_loc_array.push(first);
                    second_loc_array.push(second);
                }
                _ => {
                    eprintln!("Error: Failed to parse integers at line {}", line_number + 1);
                    return;
                }
            }
        }

        first_loc_array.sort_unstable();
        second_loc_array.sort_unstable();

        let sum = sum_distance(&first_loc_array, &second_loc_array);
        println!("Sum is: {sum}");

        let part_2 = calculate_similarity_score(&first_loc_array, &second_loc_array);
        println!("Part 2: {part_2}");
    } else {
        eprintln!("Error: Could not read the file");
    }
}

fn sum_distance(first_loc_array: &[i32], second_loc_array: &[i32]) -> i32 {
    first_loc_array
        .iter()
        .zip(second_loc_array)
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calculate_similarity_score(first_loc_array: &[i32], second_loc_array: &[i32]) -> i32 {
    first_loc_array
        .iter()
        .enumerate()
        .map(|(i, &val)| {
            second_loc_array
                .iter()
                .filter(|&&x| x == val)
                .count() as i32
                * val
        })
        .sum()
}
```

```text
Key Changes
Error Handling:

Added meaningful error messages for invalid lines or parsing issues.
Vec::push:

Used push instead of insert since elements are added sequentially.
Iterator Usage:

Simplified sum_distance and calculate_similarity_score using iterators.
Sorting Optimization:

Used sort_unstable for potentially faster sorting as it doesn't guarantee stability, which isn't required here.
Slices in Functions:

Changed function parameters from &Vec<i32> to &[i32] to make the code more flexible.
Benefits
Improved readability and maintainability.
More idiomatic and efficient Rust.
Better error handling for robust execution.
```