#[path = "../build/codegen.rs"] mod codegen;

// std
use std::{
	env,
	error::Error,
	fs,
	io::{self, Write},
	path::PathBuf,
	process::Command,
};

fn print_help() {
	println!(
		"\
Language codegen helper.

Usage:
  language [--out <path>]

Options:
  --out <path>        Where to write generated Rust (default: src/generated.rs).
  -h, --help          Show this help message."
	);
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut args = env::args().skip(1);
	let mut out_arg = None;

	while let Some(arg) = args.next() {
		match arg.as_str() {
			"--out" => {
				out_arg = Some(args.next().ok_or("Expected path after --out.")?);
			},
			"-h" | "--help" => {
				print_help();
				return Ok(());
			},
			_ => {
				return Err(format!("Unknown argument `{arg}`. Use --help for usage.").into());
			},
		}
	}

	let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

	let languages_html = fetch_languages()?;
	let code = codegen::generate(&languages_html)?;
	let out_path = out_arg
		.as_ref()
		.map(PathBuf::from)
		.unwrap_or_else(|| manifest_dir.join("src/generated.rs"));

	if let Some(parent) = out_path.parent() {
		fs::create_dir_all(parent)?;
	}

	if out_arg.is_some() {
		fs::write(&out_path, &code)?;
	} else {
		// Default behavior: refresh checked-in generated file.
		fs::write(&out_path, &code)?;
		// Also mirror to stdout for convenience.
		io::stdout().write_all(code.as_bytes())?;
	}

	Ok(())
}

fn fetch_languages() -> Result<String, Box<dyn std::error::Error>> {
	let output = Command::new("curl")
		.arg("-sSL")
		.arg("https://translation.io/docs/languages_with_plural_cases")
		.output()?;

	if !output.status.success() {
		return Err("Curl failed to download language metadata.".into());
	}

	let content = String::from_utf8(output.stdout)?;

	Ok(content)
}
