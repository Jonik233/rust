use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut memory: f64 = 0.0;

    println!("Простий калькулятор на Rust:");
    println!("Можна використовувати операції: +, -, *, /");
    println!("Введіть 'exit', щоб завершити програму.");

    loop {
        println!("\nПоточний результат: {}", memory);

        let mut input = String::new();
        print!("Введіть вираз (або 'exit' для виходу): ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).expect("Не вдалося прочитати вхідні дані");
        let input = input.trim();

        if input.to_lowercase() == "exit" {
            println!("Завершення роботи.");
            break;
        }

        let tokens: Vec<&str> = input.split_whitespace().collect();
        if tokens.len() != 3 {
            println!("Помилка: Введіть вираз у форматі 'число оператор число'.");
            continue;
        }

        let num1: f64 = match f64::from_str(tokens[0]) {
            Ok(n) => n,
            Err(_) => {
                println!("Помилка: Перше значення не є числом.");
                continue;
            }
        };

        let operator = tokens[1];

        let num2: f64 = match f64::from_str(tokens[2]) {
            Ok(n) => n,
            Err(_) => {
                println!("Помилка: Друге значення не є числом.");
                continue;
            }
        };

        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Помилка: Ділення на нуль.");
                    continue;
                }
                num1 / num2
            }
            _ => {
                println!("Помилка: Невідомий оператор '{}'.", operator);
                continue;
            }
        };

        println!("Результат: {}", result);
        memory = result;
    }
}