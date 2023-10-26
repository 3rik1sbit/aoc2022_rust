pub mod day1 {
    use std::fs::read_to_string;

    pub(crate) fn read_lines(filename: &str) -> () {
        let mut elves: Vec<i32> = Vec::new();
        let mut i = 0;
        elves.push(0);

        for line in read_to_string(filename).unwrap().lines() {
            match line.parse::<i32>() {
                Ok(n) => elves[i] += n,
                Err(_) => {
                    elves.push(0);
                    i += 1;
                }
            }
        }
        let mut total_calories = 0;

        for i in 0..3 {
            let max_value = *elves.iter().max().unwrap();
            println!("Calories of elf #{}: {}", i + 1, max_value);
            total_calories += max_value;
            let index_of_max_value = elves.iter().position(|x| *x == max_value).unwrap();
            elves.remove(index_of_max_value);
        }

        println!("Calories of three strongest elves: {}", total_calories);
    }
}