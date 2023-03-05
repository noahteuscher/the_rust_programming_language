fn main() {
    println!("\nTEMPERATURES\n");
    let temperatures = [0., 45., 78., 100.];
    for temperature in temperatures {
        println!(
            "{temperature} degrees farenheit is equivalent to {} degrees celsius.",
            farenheit_to_celsius(temperature)
        );
    }

    println!("\nFIBONACCI SEQUENCE\n");
    for fibonacci_index in 0..10 {
        println!(
            "Index {fibonacci_index} in the fibonacci sequence: {}",
            nth_fibonacci_number(fibonacci_index)
        );
    }

    println!("\nTWELVE DAYS OF CHRISTMAS\n");
    twelve_days_of_christmas();
}

fn farenheit_to_celsius(farenheit_temperature: f64) -> f64 {
    (5. / 9.) * (farenheit_temperature - 32.)
}

fn nth_fibonacci_number(fibonacci_index: u32) -> u32 {
    match fibonacci_index {
        0 => 0,
        1 => 1,
        _ => nth_fibonacci_number(fibonacci_index - 1) + nth_fibonacci_number(fibonacci_index - 2),
    }
}

fn twelve_days_of_christmas() {
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for day in 1..=12 {
        let suffix = match day {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };
        println!("On the {day}{suffix} day of Christmas, my true love sent to me");
        for previous_day in (0..day).rev() {
            println!("{}", gifts[previous_day]);
        }
        println!();
    }
}
