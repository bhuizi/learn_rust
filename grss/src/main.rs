use failure::ResultExt;
use exitfailure::ExitFailure;
use structopt::StructOpt;

#[derive(StructOpt)]

struct Cli {
    path: std::path::PathBuf
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file: {:?}", args.path))?;
    
    println!("file content: {}", content);
    Ok(())
}
