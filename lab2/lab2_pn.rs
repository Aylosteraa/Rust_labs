use std::io;
use std::str::FromStr;

fn main() {
    loop {
        println!("Введіть вираз у польській нотації (префіксній), або введіть 'exit' для виходу:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "exit" {
            break;
        }

        let tokens: Vec<&str> = input.split_whitespace().collect();

        match evaluate_prefix(&tokens) {
            Some(result) => println!("Результат: {}", result),
            None => println!("Помилка: невірний вираз!"),
        }
    }
}


fn evaluate_prefix(tokens: &[&str]) -> Option<f64> {
    
    let mut stack: Vec<f64> = Vec::new();

    for &token in tokens.iter().rev() {
        if let Ok(num) = f64::from_str(token) {

            stack.push(num);
        } else {

            let op1 = stack.pop()?;
            let op2 = stack.pop()?;

            let result = match token {
                "+" => op1 + op2,
                "-" => op1 - op2,
                "*" => op1 * op2,
                "/" => {
                    if op2 == 0.0 {
                        println!("Помилка: ділення на нуль!");
                        return None;
                    }
                    op1 / op2
                }
                _ => {
                    println!("Помилка: невідомий оператор {}", token);
                    return None;
                }
            };

            stack.push(result);
        }
    }

    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}
