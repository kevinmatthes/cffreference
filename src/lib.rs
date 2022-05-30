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

/// The application's behaviour.
pub struct Application {
    configuration: Configuration,
    description: String,
    name: String,
    usage: String,
    version: Version,
}

impl Application {
    /// Create a new application with the configured behaviour.
    pub fn new(
        configuration: &Configuration,
        options: &getopts::Options,
        version: &Version,
    ) -> Self {
        let name = String::from("cffreference");
        let synopsis = String::from("[OPTIONS]");
        let usage = options.usage(&format!("Usage:  {name} {synopsis}"));

        Self {
            configuration: configuration.clone(),
            description: String::from(
                "A simple CLI to extract citation information from a CITATION.cff.",
            ),
            name,
            usage,
            version: version.clone(),
        }
    }

    /// Run the application with the configured behaviour.
    pub fn run(&self) {
        if self
            .configuration
            .matches()
            .opt_present(self.configuration.trigger().version())
        {
            self.version();
        } else {
            self.usage();
        }
    }

    /// A brief in-app documentation.
    ///
    /// This function will write a brief usage information, including a short
    /// introduction to the meaning of the configured `options`, to `stdout`.
    fn usage(&self) {
        println!(
            "{}, version {}.{}.{}.\n{}\n\n{}",
            self.name,
            self.version.major(),
            self.version.minor(),
            self.version.fix_level(),
            self.description,
            self.usage
        );
    }

    /// A brief version information function.
    ///
    /// This function will write a short version information to `stdout`.
    fn version(&self) {
        println!(
            "{}.{}.{}",
            self.version.major(),
            self.version.minor(),
            self.version.fix_level()
        );
    }
}

/// This application's configuration.
///
/// When calling this application with command line options, its environment and
/// resulting behaviour will be set.  This struct controls which functionality
/// will be invoked.
#[derive(Clone)]
pub struct Configuration {
    matches: getopts::Matches,
    trigger: Trigger,
}

impl Configuration {
    /// The getter for the `matches` field.
    ///
    /// This function returns a read-only reference to the `matches` of the
    /// configured the options against the command line arguments.
    pub fn matches(&self) -> &getopts::Matches {
        &self.matches
    }

    /// Parse a new configuration from the command line arguments.
    ///
    /// In case that parsing should fail due to some reasons, the according
    /// error result will be returned immediately.
    pub fn parse(
        args: std::env::Args,
        options: &getopts::Options,
        trigger: &Trigger,
    ) -> Result<Self, getopts::Fail> {
        let matches = match options.parse(args) {
            Ok(matches) => matches,
            Err(error) => {
                return Err(error);
            }
        };

        Ok(Self {
            matches,
            trigger: trigger.clone(),
        })
    }

    /// The getter for the `trigger` field.
    pub fn trigger(&self) -> &Trigger {
        &self.trigger
    }
}

/// The option's names which trigger a certain mode.
#[derive(Clone)]
pub struct Trigger {
    help: String,
    version: String,
}

impl Trigger {
    /// The getter for the `help` mode trigger.
    pub fn help(&self) -> &str {
        &self.help
    }

    /// Create a new set of trigger options.
    pub fn new(help: &str, version: &str) -> Self {
        Self {
            help: String::from(help),
            version: String::from(version),
        }
    }

    /// The getter for the `version` mode trigger.
    pub fn version(&self) -> &str {
        &self.version
    }
}

/// The version information.
#[derive(Clone)]
pub struct Version {
    major: u32,
    minor: u32,
    fix_level: u32,
}

impl Version {
    /// Retrieve the fix level.
    pub fn fix_level(&self) -> &u32 {
        &self.fix_level
    }

    /// Retrieve the major version.
    pub fn major(&self) -> &u32 {
        &self.major
    }

    /// Retrieve the minor version.
    pub fn minor(&self) -> &u32 {
        &self.minor
    }

    /// Construct a new version information.
    pub fn new(major: u32, minor: u32, fix_level: u32) -> Self {
        Self {
            major,
            minor,
            fix_level,
        }
    }
}
