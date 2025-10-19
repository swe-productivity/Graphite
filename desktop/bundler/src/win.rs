use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const PACKAGE_NAME: &str = "graphite-desktop-platform-mac";
const APP_BIN_FEATURE: &str = "app";

const APP_NAME: &str = "Graphite Editor";
const APP_EXECUTABLE_NAME: &str = "graphite-editor.exe";

pub fn main() -> Result<(), Box<dyn Error>> {
	if cfg!(not(target_os = "windows")) {
		panic!("This bundler is only for Windows");
	}

	let mut profile = env!("CARGO_PROFILE");
	let profile_path = PathBuf::from(env!("CARGO_WORKSPACE_DIR")).join(format!("target/{profile}"));
	if profile == "debug" {
		profile = "dev";
	}

	let app_bin = build_bin(APP_BIN_FEATURE, profile, &profile_path)?;

	let executable = bundle(&profile_path, &app_bin);

	// TODO: Consider adding more useful cli
	if std::env::args().any(|a| a == "open") {
		let executable_path = executable.to_string_lossy();
		run_command(&executable_path, &[]).expect("failed to open app")
	}

	Ok(())
}

fn build_bin(feature: &str, profile: &str, profile_path: &Path) -> Result<PathBuf, Box<dyn Error>> {
	run_command("cargo", &["build", "--package", PACKAGE_NAME, "--profile", profile, "--no-default-features", "--features", feature]).unwrap();
	let bin_path = profile_path.join(format!("{PACKAGE_NAME}-{feature}.exe"));
	fs::copy(profile_path.join(format!("{PACKAGE_NAME}.exe")), &bin_path).unwrap();
	Ok(bin_path)
}

fn bundle(out_dir: &Path, app_bin: &Path) -> PathBuf {
	let app_dir = out_dir.join(APP_NAME);

	if app_dir.exists() {
		fs::remove_dir_all(&app_dir).unwrap();
	}
	fs::create_dir_all(&app_dir).unwrap();

	copy_cef(&app_dir);

	let bin_path = app_dir.join(APP_EXECUTABLE_NAME);

	fs::copy(app_bin, &bin_path).unwrap();

	bin_path
}

fn copy_cef(app_dir: &Path) {
	let cef_src = PathBuf::from(std::env::var("CEF_PATH").expect("CEF_PATH needs to be set"));
	copy_directory(&cef_src, app_dir);
}
