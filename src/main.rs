use clap::Parser;
mod xcap;

fn main() {
    let args = Args::parse();

    if args.fullscreen {
        xcap::fullscreen_shot();
    }
    else if args.area {
        xcap::region_screenshot();
    }
    else {
        println!("Invalid flag")
    }
}

#[derive(Parser)]
struct Args {
    #[arg(long)]
    area: bool,

    #[arg(long)]
    fullscreen: bool,
}