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

/// The main function.
///
/// This main function takes care about the application's in- and output and
/// composes its overall behaviour.  The therefore required components are
/// outsourced to the library crate root of this project.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let trigger = cffreference::Trigger::default();
    let version = cffreference::Version::new(1, 0, 0);

    let mut options: getopts::Options = getopts::Options::new();
    options
        .optflag("h", trigger.help(), "Show this help and exit.")
        .optopt(
            "i",
            trigger.input(),
            "The input file to read from.",
            "<FILE>",
        )
        .optflag(
            "l",
            trigger.license(),
            "Show the license information and exit.",
        )
        .optopt(
            "o",
            trigger.output(),
            "The output file to write to.",
            "<FILE>",
        )
        .optflag(
            "v",
            trigger.version(),
            "Show the version information and exit.",
        );

    match cffreference::Application::new(
        &match cffreference::Configuration::parse(std::env::args(), &options, &trigger) {
            Ok(configuration) => configuration,
            Err(getopts::Fail::ArgumentMissing(string)) => {
                println!(
                    "{}",
                    options.usage(&format!(
                        "No argument for option '{string}' supplied.  \
                        Valid option calls are as follows:"
                    ))
                );
                std::process::exit(cffreference::EX_USAGE);
            }
            Err(getopts::Fail::UnrecognizedOption(string)) => {
                println!(
                    "{}",
                    options.usage(&format!(
                        "Unresolvable argument:  '{string}'.  \
                        Valid options are as follows:"
                    ))
                );
                std::process::exit(cffreference::EX_USAGE);
            }
            Err(error) => {
                return Err(error.into());
            }
        },
        &options,
        &version,
    )
    .run()
    {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}

/******************************************************************************/
