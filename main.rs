#[cfg(feature = "cli")]
mod cli {
    use clap::Parser;
    use dualhash::{dualhash1024, dualhash512_trunc};

    /// Simple CLI for DualHash
    #[derive(Parser)]
    #[command(name = "dualhash", version, about = "DualHash CLI Tool")]
    struct Args {
        /// Input string
        input: String,
        /// Output 512-bit truncated hash (XOR)
        #[arg(long)]
        trunc: bool,
    }

    pub fn run() {
        let args = Args::parse();
        let msg = args.input.as_bytes();
        let digest = if args.trunc {
            dualhash512_trunc(msg)
        } else {
            dualhash1024(msg)
        };
        println!("{}", hex::encode(digest));
    }
}

#[cfg(feature = "cli")]
fn main() {
    cli::run();
}

#[cfg(not(feature = "cli"))]
fn main() {
    println!("Enable CLI with `--features cli`");
}
