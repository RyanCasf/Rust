fn main() {
    let _name: &str = "Ryan Castro Ferreira";
    let _age: i64 = 19;

    println!("$ code . - Start the vs");
    println!("I'm {} - {}", _name, _age);

    println!("\n------------\n");

    println!("Somando 1 + 2 = {}", sum(1, 2));
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}
