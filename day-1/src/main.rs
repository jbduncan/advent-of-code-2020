use itertools::Itertools;
use std::{env::args, fs::File, io, io::BufRead, path::Path};

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}

fn run_app() -> Result<(), String> {
    let expense_report_file = args()
        .nth(1)
        .ok_or("Please pass in a file name pointing to the expense report as the 1st argument.")?;
    let num_expenses_to_find = args()
        .nth(2)
        .ok_or("Please pass in a number for how many expenses should add up to 2020 as the 2nd argument.")?
        .parse::<usize>()
        .map_err(|_| "The value for how many expenses should add up to 2020 is not a whole non-negative number.")?;

    let expense_report_lines = match read_lines(&expense_report_file) {
        Ok(expense_report_lines) => expense_report_lines,
        Err(_) => return Err(format!(r#"File "{}" cannot be found."#, expense_report_file)),
    };

    let mut expenses = Vec::new();
    for (index, line) in expense_report_lines.enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(_) => return Err(format!("Line {} cannot be read.", index)),
        };

        let line = line.trim().to_string();
        if !line.is_empty() {
            expenses.push(match line.parse::<i64>() {
                Ok(expense) => expense,
                Err(_) => {
                    return Err(format!(
                        r#"Line {} is not a whole number: "{}""#,
                        index, line
                    ))
                }
            });
        }
    }

    let wanted_expenses = expenses
        .iter()
        .combinations(num_expenses_to_find)
        .find(|expense_combo| expense_combo.iter().map(|&e| e).sum::<i64>() == 2020);

    return if let Some(wanted_expenses) = wanted_expenses {
        println!("{:?}", wanted_expenses.iter().map(|&e| e).product::<i64>());
        Ok(())
    } else {
        Err(format!(
            r#"There are no expenses in "{}" that add up to 2020."#,
            expense_report_file
        ))
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
