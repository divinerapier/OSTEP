use std::io::Read;

pub struct Shell {}

impl Shell {
    pub fn new() -> Shell {
        Shell {}
    }

    pub fn run(&self) {
        let mut status = 0;
        loop {
            let current_dir = std::env::current_dir().unwrap();
            print!("{:?} C: {}>", current_dir, status);
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer).unwrap();
            let buffer = buffer.replace("  ", " ");
            let buffer = buffer.trim();
            // let args: Vec<String> = buffer.split(" ");
            let args: &[&str] = &buffer.split_ascii_whitespace().collect::<Vec<&str>>();
            if args.len() == 0 {
                continue;
            }
            status = self.exec(args[0], &args[1..]);
        }
    }

    pub fn exec(&self, cmd: &str, args: &[&str]) -> i32 {
        let output = std::process::Command::new(cmd)
            .args(args)
            .output()
            .expect("failed to execute process");
        let status = output.status.code().or_else(|| Some(255)).unwrap();
        if output.stderr.len() != 0 {
            println!("{:?}", String::from_utf8_lossy(&output.stderr));
        } else if output.stdout.len() != 0 {
            println!("{:?}", String::from_utf8_lossy(&output.stdout));
        }
        return status;
    }
}
