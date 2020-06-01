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

#[test]

fn find_match() {
    let mut result = Vec::new();
    lib::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
