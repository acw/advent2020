use advent2020::errors::TopLevelError;
use std::collections::BTreeSet;
use std::env;
use std::fs;

fn main() -> Result<(), TopLevelError> {
    let mut customs_forms_any = Vec::new();
    let mut customs_forms_all = Vec::new();

    for argument in env::args().skip(1) {
        let contents = fs::read_to_string(argument)?;
        let mut current_form_any = BTreeSet::new();
        let mut current_form_all = every_seat();

        for line in contents.lines() {
            if line.is_empty() {
                customs_forms_any.push(current_form_any);
                customs_forms_all.push(current_form_all);
                current_form_any = BTreeSet::new();
                current_form_all = every_seat();
            } else {
                let mut person_answers = BTreeSet::new();

                for char in line.chars() {
                    person_answers.insert(char);
                }

                current_form_any = current_form_any.union(&person_answers).cloned().collect();
                current_form_all = current_form_all
                    .intersection(&person_answers)
                    .cloned()
                    .collect();
            }
        }
        customs_forms_any.push(current_form_any);
        customs_forms_all.push(current_form_all);
    }

    let mut sum_all = 0;
    let mut sum_any = 0;

    for (form_any, form_all) in customs_forms_any.iter().zip(customs_forms_all.iter()) {
        println!(
            "Form: any |{}| / all |{}|",
            display_form(form_any),
            display_form(form_all)
        );
        sum_any += form_any.len();
        sum_all += form_all.len();
    }

    println!("Checked boxes (any): {}", sum_any);
    println!("Checked boxes (all): {}", sum_all);

    Ok(())
}

fn display_form(x: &BTreeSet<char>) -> String {
    let mut retval = String::new();

    for char in x.iter() {
        retval.push(*char);
    }

    retval
}

fn every_seat() -> BTreeSet<char> {
    let mut result = BTreeSet::new();

    for c in 'a'..='z' {
        result.insert(c);
    }

    result
}