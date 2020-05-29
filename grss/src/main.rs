use failure::ResultExt;
use exitfailure::ExitFailure;
use structopt::StructOpt;

#[derive(StructOpt)]

struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in  content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }

}
fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file: {:?}", args.path.display()))?;
    
    find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}

#[test]

fn find_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
