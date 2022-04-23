use std::io;

fn main() {
    println!("Please enter what you want to convert : 1(f to c), 2(c to f)");

    let mut conversion_type = String::new();
    io::stdin()
        .read_line(&mut conversion_type)
        .expect("Failed to read line");

    let conversion_type_int: i8 = conversion_type.trim().parse().ok().expect("sry");


    println!("Please enter the temperature for the conversion:");

    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp_int: f64 = temp.trim().parse().ok().expect("sry");

    if conversion_type_int == 1 {
        far(temp_int);
        // println!("{} F ", temp_int)
    } else {
        cels(temp_int);
        // println!("{} C", temp_int)
    }
}

fn cels(x: f64) {
    let y = (x * (9 as f64 / 5 as f64)) + 32 as f64;
    return println!("{:.2}", y);
}

fn far(x: f64) {
    let y = (x - 32 as f64) * (5 as f64 / 9 as f64);
    println!("{:.2}", y)
}
