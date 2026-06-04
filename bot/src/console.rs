//! Cross-platform console presentation for the bot's startup screen.
//!
//! Everything the operator sees *before* the swarm starts (the banner, the
//! first-run setup questions, the account menu, the connection steps) is drawn
//! through here so it looks identical on Windows, macOS, and Linux.
//!
//! On Windows we switch the console to UTF-8 and turn on ANSI/"virtual
//! terminal" processing once at startup; without that, modern color codes and
//! the box-drawing characters in the banner would show up as raw escape
//! sequences. On macOS and Linux those are already on, so [`init`] does nothing.

use std::io::{self, Write};

// ANSI styles. After `init()` these render on every supported terminal.
pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const PURPLE: &str = "\x1b[95m";
pub const CYAN: &str = "\x1b[96m";
pub const GREEN: &str = "\x1b[92m";
pub const YELLOW: &str = "\x1b[93m";
pub const RED: &str = "\x1b[91m";
pub const GRAY: &str = "\x1b[90m";

/// Inner width of the banner box (characters between the vertical borders).
const BOX_WIDTH: usize = 58;

/// Turn on colors and UTF-8 output. Call once, at the very start of `main`.
pub fn init() {
    #[cfg(windows)]
    // SAFETY: these console calls only read/replace this process's own console
    // mode and code page. They run once at startup before any other output.
    unsafe {
        use windows_sys::Win32::System::Console::{
            ENABLE_VIRTUAL_TERMINAL_PROCESSING, GetConsoleMode, GetStdHandle, STD_OUTPUT_HANDLE,
            SetConsoleMode, SetConsoleOutputCP,
        };
        // 65001 = UTF-8, so the banner's box-drawing characters render cleanly.
        SetConsoleOutputCP(65001);
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut mode: u32 = 0;
        if GetConsoleMode(handle, &mut mode) != 0 {
            SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }
}

/// One framed line: the given text, left-padded to the box width, with colored
/// borders. `text` must be plain (no ANSI codes) so the padding lines up.
fn framed(text: &str) {
    let pad = BOX_WIDTH.saturating_sub(text.chars().count());
    println!(
        "{PURPLE}{BOLD}║{RESET} {text}{:pad$} {PURPLE}{BOLD}║{RESET}",
        "",
        pad = pad
    );
}

/// Print the startup banner with the program name and version.
pub fn banner(version: &str) {
    let bar = "═".repeat(BOX_WIDTH + 2);
    println!();
    println!("{PURPLE}{BOLD}╔{bar}╗{RESET}");
    framed("AFK COMPANION");
    framed("Headless Minecraft companion bot · powered by Azalea");
    framed(&format!("Minecraft 1.21.11   ·   version {version}"));
    println!("{PURPLE}{BOLD}╚{bar}╝{RESET}");
    println!();
}

/// A section header, e.g. "Accounts" or "Connecting".
pub fn section(title: &str) {
    println!("{CYAN}{BOLD}▸ {title}{RESET}");
}

/// A neutral information line (two-space indent under a section).
pub fn info(text: &str) {
    println!("  {text}");
}

/// A success line (green check).
pub fn ok(text: &str) {
    println!("  {GREEN}✓{RESET} {text}");
}

/// A warning line (yellow).
pub fn warn(text: &str) {
    println!("  {YELLOW}!{RESET} {text}");
}

/// An error line (red).
pub fn error(text: &str) {
    println!("  {RED}✗{RESET} {text}");
}

/// A dimmed hint line.
pub fn hint(text: &str) {
    println!("  {GRAY}{text}{RESET}");
}

/// Ask a question on one line and read the trimmed answer. Used by the
/// first-run setup and the account menu so every prompt looks the same.
pub fn ask(question: &str) -> String {
    print!("  {PURPLE}{question}{RESET} ");
    let _ = io::stdout().flush();
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
    line.trim().to_string()
}
