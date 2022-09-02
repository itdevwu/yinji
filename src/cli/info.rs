/* Copyright 2022 Zhenglong WU (itdevwu)

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

/*!
Mod cli::info contains information of the package's name, authors & version.

It also provieds methods that print such info.
*/

use chrono::Utc;

/// Get string of package name & version
pub fn pkg_info() -> String {
    format!(
        "{} version: {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )
    .to_string()
}

/// Get string of copyright info
pub fn copyright_info() -> String {
    let current_year = Utc::now().format("%Y").to_string();
    if &current_year != "2022" {
        format!(
            "Copyright (c) 2022-{} {}. \nAll rights reserved.",
            current_year,
            env!("CARGO_PKG_AUTHORS")
        )
        .to_string()
    } else {
        format!(
            "Copyright (c) {} {}. \nAll rights reserved.",
            current_year,
            env!("CARGO_PKG_AUTHORS")
        )
        .to_string()
    }
}

/// Print package name & version
pub fn print_pkg_info() {
    println!("{}", pkg_info());
}

/// Print copyright info
pub fn print_copyright_info() {
    println!("{}", copyright_info());
}
