use failure::ResultExt;
use exitfailure::ExitFailure;
use structopt::StructOpt;

mod lib;

#[derive(StructOpt)]

struct Cli {
    pattern: String,
    path: std::path::PathBuf
}


fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file: {:?}", args.path.display()))?;
    
    lib::find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}

