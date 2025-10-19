fn main() {
	let profile = std::env::var("CARGO_PROFILE").or_else(|_| std::env::var("PROFILE")).unwrap();
	println!("cargo:rustc-env=CARGO_PROFILE={profile}");

	#[cfg(all(target_os = "windows", not(feature = "bundle")))]
	{
		let mut res = winres::WindowsResource::new();
		res.set_icon("../../assets/graphite-icon-color.ico");
		res.compile().expect("Failed to compile Windows resources");
	}
}
