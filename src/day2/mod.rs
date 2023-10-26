pub mod day2 {
    use std::fs::read_to_string;

    pub(crate) fn read_lines(filename: &str) -> () {
        let mut points = 0;
        for line in read_to_string(filename).unwrap().lines() {
            match line {
                // enemy, me R=1, P=2, S=3 || W=6, D=3, L=0
                "A X" => points += 3, // Rock, Lose (Scissors)
                "A Y" => points += 4, // Rock, Draw (Rock)
                "A Z" => points += 8, // Rock, Win (Paper)
                "B X" => points += 1, // Paper, Lose (Rock)
                "B Y" => points += 5, // Paper, Draw (Paper)
                "B Z" => points += 9, // Paper, Win (Scissors)
                "C X" => points += 2, // Scissors, Lose (Paper)
                "C Y" => points += 6, // Scissors, Draw (Scissors)
                "C Z" => points += 7, // Scissors, Win (Rock)
                _ => {}
            }
        }
        println!("Total points: {}", points)
    }
}
