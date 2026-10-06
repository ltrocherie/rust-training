/*
    Rust shenanigans
 */

use std::io;
use std::io::Write;
use rand;
use std::env;
use std::fs;

fn integers() {
    let mut car: u8 = 255;
    println!("Hello, world! The world is {}", car);
    car -= 5;
    println!("The world is now {}", car);
    let toto: f32 = 0.5;
    println!("Totot is {}", toto);

    let a= 3.0;
    let b = 10;
    let c = (b as f64 + car as f64) / a;
    println!("c is {c:08.3}\na is {a:.1}");
}

fn bits() {
    let bits: u8 = 0b1111_0101;
    let hex: u32 = 0xAB5C;
    println!("bit is {bits:08b} ({bits}) and hex is {hex}");

    let not = !bits;
    println!("not is {not:08b}");

    let and = bits & 0b0100_0000;
    println!("and is {and:08b}");

    let or = bits | 0b0000_1010;
    println!("or is {or:08b}");

    let xor = bits ^ 0b0110_0000;
    println!("xor is {xor:08b}");

    let shift = bits << 5;
    println!("shift is {shift:08b}");
}

fn booleans() {
    let yes = true;

    let no = false;

    if yes || no {

        println!("It is true!")

    } else if no {

        // Never true

    }
}

fn chars() {
    let letter = 'R';
    let number = '1';
    println!("letter is {letter} and number is {number}");
    let sign = '\u{261D}';
    println!("Hey! {sign}");
}

fn average() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = ((a as f64) + b + (c as f64)) / 3.0;
    assert_eq!(average, 45.1);
    println!("average is indeed {average:.5}");
}

fn arrays() {
    let mut array = [0, 1, 2];
    let mut first = array[0];
    println!("first value is {first}");
    array[0] += 5;
    first = array[0];
    println!("first value is now {first}");

    let numbers: [i32; 5];
    numbers = [0; 5];
    println!("last is {}", numbers[numbers.len() - 1]);

    let parking_lot = [[0, 1, 2], [0, 2, 6]];
    let number = parking_lot[0][1];
    println!("The number is {number}");

    let building = [[[0; 100]; 5]; 6];
    println!("The building size is {}", building.len());
}

fn tuples() {
    let mut tuple: (i32, char) = (0, 'R');
    tuple.0 += 3;
    println!("The first item is {}", tuple.0);

    let (a, b) = tuple;
    println!("Split: {a} and {b}");
}

fn display(number: i32) {
    println!("Number is {number}");
}

fn sum(a: u8, b: u8) {
    let sum = a + b;
    println!("Sum is {sum}");
}

fn square(a: i32) -> i32 {
    return a*a;
}

fn square_with_og(a: i32) -> (i32, i32) {

    return (a, a*a);

}

fn convert_temp(celsius: f64) -> f64 {
    return (1.8 * celsius) + 32.0;
}

fn conditions() {
    let w = 5;
    if w == 3 {
        println!("w is 3");
    } else if w > 3 {
        println!("w is greater than 3");
    } else {
        println!("w is lower than 3");
    }

    let toto = if w == 5 {6} else {7};
    println!("toto is {toto}")
}

fn loops() {

    let mut count = 0;

    let result = loop {
        count += 1;
        println!("count is {count}");
        if count == 10 {
            break count / 2;
        }
    };
    println!("result is {result}");

    let array = [0, 1, 2];
    let mut index = 0;
    while index < array.len() {
        println!("table of 5 is {}", index * array[index]);
        index += 1;
    }

    for l in ['h', 'i', '!'] {
        print!("{l}");
    }
    print!(" L\n");

    for (i, &v) in array.iter().enumerate() {
        println!("item {i} is {v}");
    }

    for n in 0..result {
        println!("range value is {n}");
    }
}

fn array_stat(array: [i32; 5]) -> (i32, i32, f64) {
    let mut min = array[0];
    let mut max = array[0];
    let mut avg = array[0] as f64;
    for n in array {
        if n < min {
            min = n;
        } else if n > max {
            max = n;
        }
        avg += n as f64;
    }
    avg /= array.len() as f64;
    return (min, max, avg);
}

fn strings() {
    let message = "nfjerngiuerniu";
    println!("message is {message}");
    let mut message = String::from("Earth is great");
    message.push_str(" and is home");
    println!("message is now: {message}");
}

fn ownership() {
    let brand: String;
    let brand_clone: String;
    {
        let inner_brand = String::from("Mercedes");
        brand_clone = inner_brand.clone();
        brand = inner_brand;
    }
    println!("brand is {brand}");
    println!("brand clone is {brand_clone}");
}

fn burn_fuel(mut fuel: String) {
    println!("Burning fuel {fuel}....");
    fuel.push_str(" burnt");
    println!("Fuel {fuel}");
}

fn process_fuel(fuel: &mut String) {
    println!("Processing fuel {fuel}....");
    fuel.push_str(" processed");
    println!("Fuel {fuel}");
}

fn slices() {
    let message = "Hello from Toulouse";
    let city = &message[11..];
    println!("City is {city}");
}

fn trim(s: &str) -> &str {
    let mut result = s;
    while result.starts_with(' ') {
        result = &result[1..];
    }
    while result.ends_with(' ') {
        result = &result[..result.len() - 1];
    }
    return result;
}

fn read_input() {
    let mut input_buffer = String::new();
    println!("What is on your mind?");
    let _result = io::stdin().read_line(&mut input_buffer);
    println!("You said: {input_buffer}");

    let mut number_buffer = String::new();
    println!("Enter a number:");
    let _result = io::stdin().read_line(&mut number_buffer);
    let number: i32 = number_buffer.trim().parse().unwrap();
    println!("Number square: {}", number * number);
}

fn random_numbers() {
    let number = rand::random::<f64>();
    println!("Random number generated: {number}");
}

fn higher_or_lower() {
    let number: i32 = rand::random_range(1..=100);
    let mut guess: i32;
    println!("I chose a number between 1 and 100. It is your duty to find it.");
    loop {
        println!("Enter your guess:");
        let mut input_buffer = String::new();
        let _result = io::stdin().read_line(&mut input_buffer);
        guess = input_buffer.trim().parse().unwrap();
        if guess > number {
            print!("Too high! ");
        } else if guess < number {
            print!("Too low! ");
        } else {
            println!("You found it! Number was indeed {number}");
            break;
        }
    }
}

fn read_drums() {
    let drum_content = fs::read_to_string("drums.txt").unwrap();
    let mut phrase = String::from("A drum kit is made of drumsticks");
    for line in drum_content.lines() {
        phrase.push_str(", ");
        phrase.push_str(line);
    }
    phrase.push_str(" and the drummer of course!");
    println!("{phrase}");
    fs::write("speech.txt", phrase);

    let mut file = fs::OpenOptions::new().append(true).open("speech.txt").unwrap();
    file.write(b"\nSource: Myself");
}

fn check_if_name_in_file(file_path: &str, name: &str) {
    println!("Checking if the file {file_path} contains {name}");
    if !fs::exists(file_path).unwrap() {
        println!("The file {file_path} does not exist");
        return;
    }
    let file_content = fs::read_to_string(file_path).unwrap();
    for line in file_content.lines() {
        if line.eq(name) {
            println!("Match found for the name {name}");
            return;
        }
    }
    println!("The name {name} is absent");
}

fn main() {
    integers();
    bits();
    booleans();
    chars();
    average();
    arrays();
    tuples();
    display(2);
    let x = 5;
    sum(x, 5);
    display(x.into());
    println!("Square is {}", square(13));
    let (value, square) = square_with_og(13);
    println!("Square of {value} is {square}");
    println!("39°C is {}°F", convert_temp(39.0));
    conditions();
    loops();
    let (min, max, avg) = array_stat([6, 45, 78, 6, 73]);
    println!("min: {min}, max: {max}, avg: {avg}");
    strings();
    ownership();

    let kerosene = String::from("Kerosene");
    burn_fuel(kerosene.clone());
    println!("Kerosene: {kerosene}");
    let mut gasoline = String::from("Gasoline");
    process_fuel(&mut gasoline);
    println!("Gasoline: {gasoline}");

    slices();

    let message = String::from("     Hello good sir    ");
    println!("Trim message is {}", trim(&message));
    println!("Original message is {}", message);

    // read_input();

    random_numbers();

    // higher_or_lower();
    
    for (index, arg) in env::args().enumerate() {
        println!("Argument {index} is {arg}");
    }

    read_drums();

    if env::args().len() >= 2 {
        let arg2 = env::args().nth(2).unwrap();
        println!("Argument 2 is {arg2}");
        check_if_name_in_file( &env::args().nth(1).unwrap(),  &env::args().nth(2).unwrap());
    } else {
        println!("No second argument to print");
    }

}
