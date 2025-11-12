windows::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};

#![windows_subsystem = "windows"]
fn main() {
	let _ = unsafe { AttachConsole(ATTACH_PARENT_PROCESS) };

	graphite_desktop::start();
}
