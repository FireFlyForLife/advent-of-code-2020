mod puzzle_input;

fn expense_report(expenses: &Vec<i32>) -> Option<i32> {
    for i in 0..expenses.len() {
        for j in i..expenses.len() {
            if expenses[i] + expenses[j] == 2020 {
                return Some(expenses[i] * expenses[j]);
            }
        }
    }

    None
}

fn advanced_expense_report(expenses: &Vec<i32>) -> Option<i32> {
    for i in 0..expenses.len() {
        for j in i..expenses.len() {
            for k in j..expenses.len() {
                if expenses[i] + expenses[j] + expenses[k] == 2020 {
                    return Some(expenses[i] * expenses[j] * expenses[k]);
                }
            }
        }
    }

    None
}

fn main() {
    let expenses: Vec<_> = puzzle_input::EXPENSES.lines().map(|line| line.parse::<i32>().unwrap()).collect();

    let report = expense_report(&expenses).unwrap();
    let advanced_report = advanced_expense_report(&expenses).unwrap();

    println!("Simple Expense report: {}", report);
    println!("Advanced Expense report: {}", advanced_report);
}
