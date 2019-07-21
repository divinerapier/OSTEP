mod question01;

fn main() {
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();

    // for (index, arg) in args.iter().enumerate() {
    //     println!("index: {}, arg: {}", index, arg);
    // }

    if args.len() == 1 {
        println!("nothing to be processed!");
        return;
    }

    match args[1].parse::<i32>() {
        Ok(index) => match index {
            1 => {
                question01::free_null_pointer();
            }
            index @ _ => {
                println!("no such question. {}", index);
            }
        },
        Err(e) => {
            println!("invalid argument. {}", args[1]);
            return;
        }
    }
}
