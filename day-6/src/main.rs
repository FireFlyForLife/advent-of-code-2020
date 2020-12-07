use itertools::Itertools;

mod puzzle_input;


fn main() {
    let num_unique_yes: usize = puzzle_input::ANSWERS.split("\n\n")
        .map(|group| group.chars().filter(|c| c.is_ascii_alphabetic()).unique().count())
        .sum();

    let num_common_yes: usize = puzzle_input::ANSWERS.split("\n\n")
        .map(|group| { 
            let group_passenger_answers: Vec<Vec<char>> = group.lines().map(|passenger_line| passenger_line.chars().collect()).collect();
            if group_passenger_answers.is_empty() {
                return 0;
            }
            let mut common_yes_answers = group_passenger_answers.first().unwrap().clone();
            for passenger_answers in &group_passenger_answers[1..] {
                common_yes_answers = common_yes_answers.into_iter().filter(|c| passenger_answers.contains(c)).collect();
            }
            
            common_yes_answers.len()
        })
        .sum();

    println!("Number of unique yes answers: {}", num_unique_yes);
    println!("Number of common yes answers: {}", num_common_yes);
}
