use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;

/// search for a patten in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
	/// The pattern to look for
	pattern: String,
	/// The path to the file to read
	#[structopt(parse(from_os_str))]
	path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
	let args = Cli::from_args();

	let content = std::fs::read_to_string(&args.path)
		.with_context(|_| format!("Error reading `{}`", args.path.display()))?;

	grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());
	Ok(())
}

#[test]
fn find_a_match() {
	let mut result = Vec::new();
	grrs::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
	assert_eq!(result, b"lorem ipsum\n");
}