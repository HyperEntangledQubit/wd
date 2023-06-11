// Licensed under MIT license <LICENSE-MIT or https://opensource.org/licenses/MIT>
// All files in the project carrying such notice may not be copied, modified, or
// distributed except according to those terms.

extern crate version_check as rustc;

use std::process::exit;

fn main() {
    // Ensure version of rustc isn't too low
    if rustc::is_min_version("1.64.0") != Some(true) {
        eprintln!("Hit error during build.rs");
        exit(1);
    }
}
