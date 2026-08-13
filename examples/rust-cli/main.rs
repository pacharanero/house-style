// This example will receive the repository's permissive code licence before release.

use anyhow::Result;

fn main() -> Result<()> {
    reset_sigpipe();
    example_cli::run()
}

#[cfg(unix)]
fn reset_sigpipe() {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
}

#[cfg(not(unix))]
fn reset_sigpipe() {}
