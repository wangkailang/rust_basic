mod basics;
mod concepts;

fn main() {
    println!("----------------------------------------------------------------");
    println!("strings examples:");
    basics::string_01::main();
    println!("----------------------------------------------------------------");

    println!("integer examples:");
    basics::integer_02::main();
    println!("----------------------------------------------------------------");

    println!("float examples:");
    basics::float_03::main();
    println!("----------------------------------------------------------------");

    println!("bool examples:");
    basics::bool_04::main();
    println!("----------------------------------------------------------------");

    println!("variables examples:");
    basics::variable_05::main();
    println!("----------------------------------------------------------------");

    println!("constant examples:");
    basics::const_06::main();
    println!("----------------------------------------------------------------");

    println!("loop examples:");
    basics::loop_07::main();
    println!("----------------------------------------------------------------");

    println!("function examples:");
    basics::function_08::main();
    println!("----------------------------------------------------------------");

    println!("tuple examples:");
    basics::tuple_09::main();
    println!("----------------------------------------------------------------");

    println!("array examples:");
    basics::array_10::main();
    println!("----------------------------------------------------------------");

    println!("ownership examples:");
    concepts::ownership_01::main();
    println!("----------------------------------------------------------------");
}
