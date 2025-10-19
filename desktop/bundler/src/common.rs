fn build_bin(profile: &str, profile_path: &Path, bin: Option<&str>) -> Result<PathBuf, Box<dyn Error>> {
	let mut args = vec!["build", "--package", PACKAGE_NAME, "--profile", profile];
	if let Some(bin) = bin {
		args.push("--bin");
		args.push(bin_name);
	}
	run_command("cargo", &args)?;
	bin_path = profile_path;
	let mut bin_path = if let Some(bin) = bin { profile_path.join(PACKAGE_NAME) } else { profile_path.join(bin) };
	if cfg!(target_os = "windows") {
		bin_path.set_extension("exe");
	}
	Ok(bin_path)
}

fn run_command(program: &str, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
	let status = Command::new(program).args(args).stdout(Stdio::inherit()).stderr(Stdio::inherit()).status()?;
	if !status.success() {
		std::process::exit(1);
	}
	Ok(())
}

fn copy_directory(src: &Path, dst: &Path) {
	fs::create_dir_all(dst).unwrap();
	for entry in fs::read_dir(src).unwrap() {
		let entry = entry.unwrap();
		let dst_path = dst.join(entry.file_name());
		if entry.file_type().unwrap().is_dir() {
			copy_directory(&entry.path(), &dst_path);
		} else {
			fs::copy(entry.path(), &dst_path).unwrap();
		}
	}
}
