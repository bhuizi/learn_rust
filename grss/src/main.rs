use structopt::StructOpt;

// Search for a pattern in a file and display the lines that contain it

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf
}

fn main() {
    let args = Cli::from_args();
}
