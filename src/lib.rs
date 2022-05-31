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

//! The crate root.
//!
//! This source file defines the data structures, magic numbers as well as
//! utility functions of this project.

/// The passed command line options are invalid.
pub const EX_USAGE: i32 = 64;

/// The application's behaviour.
pub struct Application {
    /// This application's `configuration`.
    configuration: Configuration,

    /// Information about this application.
    description: Description,

    /// The input file to read from.
    input: Option<String>,

    /// The output file to write to.
    output: Option<String>,

    /// The version information.
    version: Version,
}

impl Application {
    /// Read the input.
    fn preprocess(&self) -> Result<String, std::io::Error> {
        match std::fs::read_to_string(self.input.as_ref().unwrap()) {
            Ok(string) => Ok(string
                .lines()
                .filter(|x| {
                    !x.is_empty()
                        && !x.starts_with('#')
                        && !x.starts_with("---")
                        && !x.starts_with("```")
                        && !x.starts_with("cff-version:")
                        && !x.starts_with("message:")
                })
                .map(|x| String::from(x) + "\n")
                .collect()),
            Err(error) => match error.kind() {
                std::io::ErrorKind::NotFound => {
                    eprintln!("File '{}' not found.", self.input.as_ref().unwrap());
                    std::process::exit(EX_USAGE);
                }
                _ => Err(error),
            },
        }
    }

    /// Show the license information.
    fn license(&self) {
        println!(
            "\
            Copyright (C) 2022 Kevin Matthes\n\
            \n\
            This program is free software; you can redistribute it and/or modify\n\
            it under the terms of the GNU General Public License as published by\n\
            the Free Software Foundation; either version 2 of the License, or\n\
            (at your option) any later version.\n\
            \n\
            This program is distributed in the hope that it will be useful,\n\
            but WITHOUT ANY WARRANTY; without even the implied warranty of\n\
            MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n\
            GNU General Public License for more details.\n\
            \n\
            You should have received a copy of the GNU General Public License along\n\
            with this program; if not, write to the Free Software Foundation, Inc.,\n\
            51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.\
            "
        );
    }

    /// Create a new application with the configured behaviour.
    pub fn new(
        configuration: &Configuration,
        options: &getopts::Options,
        version: &Version,
    ) -> Self {
        let name = "cffreference";
        let synopsis = "[OPTIONS]";
        let usage = options.usage(&format!("Usage:  {name} {synopsis}"));

        Self {
            configuration: configuration.clone(),
            description: Description::new(
                name,
                "A simple CLI to extract citation information from a CITATION.cff.",
                synopsis,
                &usage,
            ),
            input: configuration
                .matches()
                .opt_str(configuration.trigger().input()),
            output: configuration
                .matches()
                .opt_str(configuration.trigger().output()),
            version: version.clone(),
        }
    }

    /// Run the application with the configured behaviour.
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self
            .configuration
            .matches()
            .opt_present(self.configuration.trigger().version())
        {
            self.version();
        } else if self
            .configuration
            .matches()
            .opt_present(self.configuration.trigger().license())
        {
            self.license();
        } else if self
            .configuration
            .matches()
            .opt_present(self.configuration.trigger().input())
        {
            println!(
                "Input:  {}\nOutput:  {}",
                self.input
                    .as_ref()
                    .ok_or_else(|| getopts::Fail::ArgumentMissing(String::from(
                        self.configuration.trigger().input()
                    )))?,
                if self
                    .configuration
                    .matches()
                    .opt_present(self.configuration.trigger().output())
                {
                    self.output.as_ref().ok_or_else(|| {
                        getopts::Fail::ArgumentMissing(String::from(
                            self.configuration.trigger().output(),
                        ))
                    })?
                } else {
                    "stdout"
                }
            );
            print!("{}", self.preprocess()?);
        } else {
            self.usage();
        }

        Ok(())
    }

    /// A brief in-app documentation.
    ///
    /// This function will write a brief usage information, including a short
    /// introduction to the meaning of the configured `options`, to `stdout`.
    fn usage(&self) {
        println!(
            "{}, version {}.{}.{}.\n{}\n\n{}",
            self.description.name(),
            self.version.major(),
            self.version.minor(),
            self.version.fix_level(),
            self.description.summary(),
            self.description.usage()
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
    /// The found options.
    matches: getopts::Matches,

    /// The configured triggers for the implemented modes.
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

/// The application's self description.
pub struct Description {
    /// The application's `name`.
    name: String,

    /// A brief idea what this application actually does.
    summary: String,

    /// The summary of the mandatory arguments to pass.
    synopsis: String,

    /// The description of the configured options.
    usage: String,
}

impl Description {
    /// Retrieve the application's `name`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Create a new description from the given information.
    pub fn new(name: &str, summary: &str, synopsis: &str, usage: &str) -> Self {
        Self {
            name: String::from(name),
            summary: String::from(summary),
            synopsis: String::from(synopsis),
            usage: String::from(usage),
        }
    }

    /// Retrieve the application's `summary`.
    pub fn summary(&self) -> &str {
        &self.summary
    }

    /// Retrieve the application's `synopsis`.
    pub fn synopsis(&self) -> &str {
        &self.synopsis
    }

    /// Retrieve the application's `usage` information.
    pub fn usage(&self) -> &str {
        &self.usage
    }
}

/// The option's names which trigger a certain mode.
#[derive(Clone, Debug)]
pub struct Trigger {
    /// The long option for the `help` mode.
    help: String,

    /// The `input` file to read from.
    input: String,

    /// The long option for the `license` information mode.
    license: String,

    /// The `output` file to write to.
    output: String,

    /// The long option for the `version` information mode.
    version: String,
}

impl Trigger {
    /// Retrieve the `help` mode trigger.
    pub fn help(&self) -> &str {
        &self.help
    }

    /// Retrieve the `input` file's name.
    pub fn input(&self) -> &str {
        &self.input
    }

    /// Retrieve the `license` mode trigger.
    pub fn license(&self) -> &str {
        &self.license
    }

    /// Create a new set of trigger options.
    pub fn new(help: &str, input: &str, license: &str, output: &str, version: &str) -> Self {
        Self {
            help: String::from(help),
            input: String::from(input),
            license: String::from(license),
            output: String::from(output),
            version: String::from(version),
        }
    }

    /// Retrieve the `output` file's name.
    pub fn output(&self) -> &str {
        &self.output
    }

    /// Retrieve the `version` mode trigger.
    pub fn version(&self) -> &str {
        &self.version
    }
}

impl Default for Trigger {
    fn default() -> Trigger {
        Self {
            help: String::from("help"),
            input: String::from("input"),
            license: String::from("license"),
            output: String::from("output"),
            version: String::from("version"),
        }
    }
}

impl PartialEq for Trigger {
    fn eq(&self, other: &Trigger) -> bool {
        self.help() == other.help()
            && self.input() == other.input()
            && self.license() == other.license()
            && self.output() == other.output()
            && self.version() == other.version()
    }
}

/// The version information.
#[derive(Clone)]
pub struct Version {
    /// The application's major version.
    major: u32,

    /// The application's minor version.
    minor: u32,

    /// The application's fix level.
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

/******************************************************************************/
