use clap::Parser;
use rzpipe::RzPipe;

#[derive(Debug, Clone, Parser)]
struct Opts {
    #[clap(long)]
    bin: String,
    #[clap(long)]
    cmd: String,
}

fn main() {
    let opts = Opts::parse();
    let bin = opts.bin;
    let cmd = opts.cmd;

    let mut rz = RzPipe::spawn(bin, None).unwrap();
    let _ = rz.cmd("aa").unwrap();

    let cmd_split: Vec<&str> = cmd.split('|').collect();

    let mut previous = "".to_string();
    for cmd in cmd_split {
        let new_cmd = format!("{} {}", cmd, previous);
        dbg!(&new_cmd);
        let output = rz.cmd(&new_cmd).unwrap();
        previous = output.clone();
    }
    println!("{}", previous);
}
