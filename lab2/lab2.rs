use std::io::{self, Write};

fn main() {
    let mut memory: Option<f64> = None;

    loop {
        
        println!("\nКласичний калькулятор:");
        println!("1. Додавання");
        println!("2. Віднімання");
        println!("3. Множення");
        println!("4. Ділення");
        println!("5. Показати результат з пам'яті");
        println!("6. Вийти");
        
        print!("Виберіть операцію: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<u32>();

        match choice {
            Ok(1) | Ok(2) | Ok(3) | Ok(4) => {
                let (a, b) = get_two_numbers();

                let result = match choice.unwrap() {
                    1 => a + b,
                    2 => a - b,
                    3 => a * b,
                    4 => {
                        if b == 0.0 {
                            println!("Помилка: Ділення на нуль не дозволене.");
                            continue;
                        }
                        a / b
                    }
                    _ => unreachable!(),
                };

                println!("Результат: {}", result);
                memory = Some(result); 
            }
            Ok(5) => {
                if let Some(result) = memory {
                    println!("Збережений результат: {}", result);
                } else {
                    println!("Немає результату в пам'яті.");
                }
            }
            Ok(6) => {
                println!("До побачення!");
                break;
            }
            _ => {
                println!("Неправильний вибір. Будь ласка, спробуйте ще раз.");
            }
        }
    }
}

fn get_two_numbers() -> (f64, f64) {
    loop {
        let mut input = String::new();

        print!("Введіть перше число: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let a = input.trim().parse::<f64>();

        if let Ok(a) = a {
            input.clear();

            print!("Введіть друге число: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let b = input.trim().parse::<f64>();

            if let Ok(b) = b {
                return (a, b);
            } else {
                println!("Помилка: Неправильне значення. Спробуйте ще раз.");
            }
        } else {
            println!("Помилка: Неправильне значення. Спробуйте ще раз.");
        }
    }
}
