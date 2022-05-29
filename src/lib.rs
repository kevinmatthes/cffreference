/*********************** GNU General Public License 2.0 ***********************\
|                                                                              |
|  Copyright (C) 2022 Kevin Matthes                                            |
|                                                                              |
|  This program is free software; you can redistribute it and/or modify        |
|  it under the terms of the GNU General Public License as published by        |
|  the Free Software Foundation; either version 2 of the License, or           |
|  (at your option) any later version.                                         |
|                                                                              |
|  This program is distributed in the hope that it will be useful,             |
|  but WITHOUT ANY WARRANTY; without even the implied warranty of              |
|  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the               |
|  GNU General Public License for more details.                                |
|                                                                              |
|  You should have received a copy of the GNU General Public License along     |
|  with this program; if not, write to the Free Software Foundation, Inc.,     |
|  51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.                 |
|                                                                              |
\******************************************************************************/

//! The library crate root.
//!
//! This source file defines the data structures, magic numbers as well as
//! utility functions of this project.

/// The version information.
pub mod version {
    /// This application's major version.
    pub const MAJOR: u32 = 1u32;

    /// This application's minor version.
    pub const MINOR: u32 = 0u32;

    /// This application's fix level.
    pub const FIX_LEVEL: u32 = 0u32;
}

/// A brief in-app documentation.
///
/// This function will write a brief usage information, including a short
/// introduction to the meaning of the configured `options`, to `stdout`.
pub fn usage(options: &getopts::Options) {
    let description = "A simple CLI to extract citation information from a CITATION.cff.";
    let name = "cffreference";
    let synopsis = "[OPTIONS]";

    println!(
        "{name}, version {}.{}.{}.\n{description}\n\n{}",
        crate::version::MAJOR,
        crate::version::MINOR,
        crate::version::FIX_LEVEL,
        options.usage(&format!("Usage:  {name} {synopsis}"))
    );
}

/// A brief version information function.
///
/// This function will write a short version information to `stdout`.
pub fn version() {
    println!(
        "{}.{}.{}",
        crate::version::MAJOR,
        crate::version::MINOR,
        crate::version::FIX_LEVEL
    );
}
