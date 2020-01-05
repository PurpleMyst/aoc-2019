use intcode::Interpreter;

fn main() {
    let mut interpreter = Interpreter::from_input(include_str!("input.txt"));
    interpreter.memory.extend_from_slice(&[0; 1080 - 973]);

    {
        let mut interpreter = interpreter.clone();
        interpreter.input.push_back(1);
        interpreter.run();
        println!("{:?}", interpreter.output[0]);
    }

    {
        let mut interpreter = interpreter;
        interpreter.input.push_back(2);
        interpreter.run();
        println!("{:?}", interpreter.output[0]);
    }
}
