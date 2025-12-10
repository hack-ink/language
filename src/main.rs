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
  language [--fetch] [--out <path>]

Options:
  --fetch             Refresh language metadata snapshot from Translation.io.
  --out <path>        Where to write generated Rust (default: src/generated.rs).
  -h, --help          Show this help message."
	);
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut args = env::args().skip(1);
	let mut fetch = false;
	let mut out_arg = None;

	while let Some(arg) = args.next() {
		match arg.as_str() {
			"--fetch" => fetch = true,
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
	let data_dir = manifest_dir.join("data");
	let languages_path = data_dir.join("languages_with_plural_cases.html");

	if fetch {
		fetch_languages(&languages_path)?;
	}

	let code = codegen::generate(&languages_path)?;
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

fn fetch_languages(dest: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
	if let Some(parent) = dest.parent() {
		fs::create_dir_all(parent)?;
	}

	let status = Command::new("curl")
		.arg("-sSL")
		.arg("https://translation.io/docs/languages_with_plural_cases")
		.arg("-o")
		.arg(dest)
		.status()?;

	if !status.success() {
		return Err("Curl failed to download language metadata.".into());
	}

	Ok(())
}
