mod string_01;
mod integer_02;
mod float_03;
mod bool_04;
mod variable_05;
mod const_06;
mod loop_07;

fn main() {
    println!("----------------------------------------------------------------");
    println!("strings examples:");
    string_01::main();
    println!("----------------------------------------------------------------");

    println!("integer examples:");
    integer_02::main();
    println!("----------------------------------------------------------------");

    println!("float examples:");
    float_03::main();
    println!("----------------------------------------------------------------");

    println!("bool examples:");
    bool_04::main();
    println!("----------------------------------------------------------------");

    println!("variables examples:");
    variable_05::main();
    println!("----------------------------------------------------------------");

    println!("constant examples:");
    const_06::main();
    println!("----------------------------------------------------------------");
}
