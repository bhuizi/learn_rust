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
    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => { content },
        Err(error) => { panic!("Can't deal with {}, just exit here", error);}
    };
    
    println!("file content: {}", content);
}
