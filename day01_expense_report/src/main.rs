use std::collections::HashSet;
use std::iter::FromIterator;
// .lines()
use std::io::prelude::*;

// Using the above example again, the three entries that sum to 2020 are 979, 366, and 675.
// 0(N^2)
fn find_three_matching_expenses(expenses : &HashSet<i64>) -> (i64, i64, i64) {
    for first_expense in expenses {
        for second_expense in expenses {
            let expense_remaining = 2020 - first_expense - second_expense;
            if expenses.contains(&expense_remaining) {
                return (*first_expense, *second_expense, expense_remaining);
            }
        }
    }

    return (0, 0, 0); // assume that the input always has at least 1 triple that sums to 2020
}

// Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.
// O(N)
fn find_two_matching_expenses(expenses : &HashSet<i64>) -> (i64, i64) {
    for expense in expenses {
        let expense_remaining = 2020 - expense;
        if expenses.contains(&expense_remaining) {
            return (*expense, expense_remaining);
        }
    }

    return (0, 0); // assume that the input always has at least 1 pair that sums to 2020
}

fn main() {
    let expenses : HashSet<i64> = HashSet::from_iter(
        std::io::stdin().lock().lines()
            .map(|l| l.unwrap().parse().unwrap()) // convert to i64. assume all input lines are valid i64
    );

    let (first, second) = find_two_matching_expenses(&expenses);
    // Find the two entries that sum to 2020; what do you get if you multiply them together?
    let multipled = first * second;
    println!("{}", multipled);
    let (first, second, third) = find_three_matching_expenses(&expenses);
    //Multiplying them together produces the answer, 241861950.
    let multipled = first * second * third;
    println!("{}", multipled);
}
