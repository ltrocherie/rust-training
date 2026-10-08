/*
    Rust shenanigans
 */

use std::io;
use std::io::Write;
use std::mem;
use std::fmt;
use rand;
use std::env;
use std::fs;

use crate::Location::Anonymous;
use crate::Location::Known;
use crate::Location::Unknown;

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
        let result = io::stdin().read_line(&mut input_buffer);
        match result {
            Ok(_) => (),
            Err(_) => panic!("Could not read from standard input")
        }
        guess = match input_buffer.trim().parse() {
            Ok(number) => number,
            Err(_) => { 
                println!("Not a number, try again!");
                continue;
            }
        };
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

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
struct Car {
    name: String,
    crew: u8,
    engine_capacity: f64
}

impl Car {
    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn increase_engine_capacity(&mut self, liters: f64) {
        self.engine_capacity += liters;
    }

    fn new(name: &str) -> Car {
        return Car {
            name: String::from(name),
            crew: 4,
            engine_capacity: 2.0
        };
    }
}

fn structs() {
    let mut z4 = Car {
        name: String::from("BMW Z4"),
        crew: 2,
        engine_capacity: 2.5
    };
    println!("Z4 has a {}L engine", z4.engine_capacity);

    let mut clio = Car {
        name: String::from("Renault Clio"),
        ..z4
    };

    let z4clone = Car {
        ..z4.clone()
    };

    z4.crew = 1;
    println!("Z4 is {z4:?}");
    println!("Z4 clone is {z4clone:?}");
    println!("Clio is {clio:?}");

    let clio_name = clio.get_name();
    println!("Clio name is {clio_name}");

    clio.increase_engine_capacity(1 as f64);
    println!("Modified Clio is {clio:?}");

    let scenic = Car::new("Renault Scenic");
    println!("Scenic is {scenic:?}");
}

struct Color(u8, u8, u8); // RGB

fn colors() {
    let red = Color(255, 0, 0);
    println!("First value is {}", red.0);
}

struct Rectangle<T, U> {
    width: T,
    height: U
}

impl<T, U> Rectangle<T, U> {

    fn get_width(&self) -> &T {
        return &self.width;
    }

    fn new(width: T, height: U) -> Rectangle<T, U> {
        return Rectangle {
            width: width,
            height: height
        };
    }
}

impl Rectangle<f64, f64> {
    fn get_area(&self) -> f64 {
        return self.width * self.height;
    }

    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn rectangles() {
    let mut rect: Rectangle<f64, f64> = Rectangle::<f64, f64>::new(1.2, 3.4);
    assert_eq!(*rect.get_width(), 1.2);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Rectangle is OK");
}

fn get_max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        return a;
    }
    return b;
}

fn boxes() {
    let boxed_car: Box<Car> = Box::new(Car::new("Ford Mustang"));
    println!("The Mustang takes {} bytes on the stack", mem::size_of_val(&boxed_car));
    println!("The Mustang takes {} bytes on the heap", mem::size_of_val(&*boxed_car));

    let unboxed_car = *boxed_car;
    println!("The Mustang now takes {} bytes on the stack", mem::size_of_val(&unboxed_car));
}

fn sum_boxes<T: std::ops::Add<Output = T>>(a: Box<T>, b: Box<T>) -> Box<T> {
    return Box::new(*a + *b);
}

fn test_sum_boxes() {
    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_boxes(one, two), 3);

    let pi = Box::new(3.14);
    let e = Box::new(2.71);
    assert_eq!(*sum_boxes(pi, e), 5.85);

    println!("Boxes OK");
}

trait Description {
    fn describe(&self) -> String {
        return String::from("No description");
    }
}

impl Description for Car {
    fn describe(&self) -> String {
        return format!("The car is named {}, can carry {} people and has a engine of {}L capacity", self.name, self.crew, self.engine_capacity);
    }
}

struct Satellite {
    name: String,
    velocity: f64,
    altitude: f64
}

impl Satellite {
    fn new(name: &str) -> Satellite {
        return Satellite { name: String::from(name), velocity: 4.72, altitude: 400.0 }
    }
}

impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{} flying at {}m/s and {}km high above Earth", self.name, self.velocity, self.altitude);
    }
}

fn trait_satellite() {
    let hubble = Satellite::new("Hubble Telescope");
    println!("Hubble: {}", hubble);
}

#[derive(Debug)]
enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
    Square(f64),
    Triangle(f64, f64, f64)
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        match *self {
            Shape::Circle(r) => (std::f64::consts::PI * r * 2.0) as f64,
            Shape::Rectangle(x, y) => (2.0*x + 2.0*y) as f64,
            Shape::Square(x) => 4.0*x,
            Shape::Triangle(x, y, z) => -1.0
        }
    }
}

fn enums() {
    let shape = Shape::Rectangle(1.2, 2.3);
    println!("Shape is {shape:?}");

    match shape {
        Shape::Circle(r) => println!("Is a circle"),
        Shape::Rectangle(x, y) => println!("Is a rectangle ({x}, {y})"),
        Shape::Square(x) => println!("Is a square"),
        Shape::Triangle(x, y, z) => println!("Is a triangle")
    }

    let number = 1u8;
    let result = match number {
        0 => "zero",
        _ => "not zero"
    };
    println!("Result is {result}");

    let perimeter = shape.get_perimeter();
    println!("Permimeter is {perimeter}");

    let array = [1, 2];
    let number = array.get(3).unwrap_or(&0) + 1;
    let number = match array.get(5) {
        Some(number) => number + 1,
        None => -1
    };
    println!("New number is {number:?}");

    let num = Some(13);
    if let Some(13) = num {
        println!("thirteen");
    }
}

enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64)
}

impl Location {
    fn display(&self) {
        match self {
            Unknown => println!("Location unknown"),
            Anonymous => println!("Location undisclosed"),
            Known(x, y) => println!("Individual located at {x} {y} coordinates")
        }
    }
}

fn panic() {
    // panic!("oups");
}

fn locations() {
    Location::Unknown.display();
    Location::Anonymous.display();
    Location::Known(100.42, 50.7).display();
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

    structs();
    colors();
    rectangles();

    let max = get_max(1, 2);
    println!("The max is {max}");

    boxes();
    test_sum_boxes();

    let peugeot = Car::new("Peugeot 208");
    println!("208: {}", peugeot.describe());

    trait_satellite();

    enums();
    locations();

    panic();

}
