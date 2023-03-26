use clap::{command, Parser};
use wrap::Wrapper;

#[derive(Parser)]
#[command(author="Grigory Latyshev", version ="1.0", about="Wrap stdin in columns", long_about = None)]
struct Args {
    /// Column length
    #[arg(short, long, default_value_t = 80)]
    wrap: usize,
}

fn main() {
    let args = Args::parse();
    let stdin = std::io::stdin();
    let mut buf = String::new();
    let mut wrapper = Wrapper::new(args.wrap);
    while let Ok(nbytes) = stdin.read_line(&mut buf) {
        if nbytes == 0 {
            break;
        }
        let trimmed = buf.trim_end();
        wrapper.read(&trimmed, |s| {
            println!("{}", s);
        });
        buf.clear();
    }
}
