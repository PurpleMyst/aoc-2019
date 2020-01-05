const COMPUTERS: usize = 50;
const NAT_ADDRESS: usize = 255;

use intcode::Interpreter;

fn main() {
    use std::mem::{self, MaybeUninit};

    let mut interpreter = Interpreter::from_input(include_str!("input.txt"));
    interpreter.memory.extend_from_slice(&[0; 10]);

    // To avoid having to allocate a vector on the heap, use a magic incantation to get a
    // stack-allocated array. Does this affect performance? Probably not. Did I want to do it for
    // fun? yes!
    let mut computers = {
        let mut computers: [MaybeUninit<Interpreter>; COMPUTERS] =
            unsafe { MaybeUninit::uninit().assume_init() };

        computers.iter_mut().for_each(|elem| {
            *elem = MaybeUninit::new(interpreter.clone());
        });

        unsafe { mem::transmute::<_, [Interpreter; COMPUTERS]>(computers) }
    };

    // Send every computer its network ID
    for i in 0..COMPUTERS {
        computers[i].input.push_back(i as i64);
    }

    // What was the last packet sent to the NAT?
    let mut nat: Option<(i64, i64)> = None;

    // What was the last Y sent out by the NAT?
    let mut last_y: Option<i64> = None;

    loop {
        for i in 0..COMPUTERS {
            if computers[i].input.is_empty() {
                computers[i].input.push_back(-1);
            }

            computers[i].run();

            while !computers[i].output.is_empty() {
                let mut it = computers[i].output.drain(..3);
                let address = it.next().unwrap() as usize;
                let x = it.next().unwrap();
                let y = it.next().unwrap();

                if address == NAT_ADDRESS {
                    nat = Some((x, y));
                } else {
                    computers[address].input.push_back(x);
                    computers[address].input.push_back(y);
                }
            }
        }

        if (0..COMPUTERS).all(|i| computers[i].input.is_empty()) {
            if let Some((x, y)) = nat.take() {
                if last_y == None {
                    println!("{}", y);
                } else if last_y == Some(y) {
                    println!("{}", y);
                    return;
                }
                last_y = Some(y);

                computers[0].input.push_back(x);
                computers[0].input.push_back(y);
            }
        }
    }
}
