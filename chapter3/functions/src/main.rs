fn main() {
    print_labeled_measurement(5, 'h');
    statements_and_expressions();
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn statements_and_expressions() {
    let y = {
        let x = five();
        x + 1
    };
    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}
