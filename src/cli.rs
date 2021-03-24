use std::{convert::TryFrom, path::PathBuf, vec::Vec};

use clap::{Arg, Values};
// use colored::Colorize;

use crate::error;
use crate::extension::Extension;
use crate::file::File;

#[derive(PartialEq, Eq, Debug)]
pub enum CommandKind {
    Compression(
        // Files to be compressed
        Vec<PathBuf>,
    ),
    Decompression(
        // Files to be decompressed and their extensions
        Vec<File>,
    ),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Command {
    pub kind: CommandKind,
    pub output: Option<File>,
}

pub fn clap_app<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new("ouch")
        .version("0.1.2")
        .about("ouch is a unified compression & decompression utility")
        .after_help(
"ouch infers what to based on the extensions of the input files and output file received.
Examples: `ouch -i movies.tar.gz classes.zip -o Videos/` in order to decompress files into a folder.
          `ouch -i headers/ sources/ Makefile -o my-project.tar.gz` 
          `ouch -i image{1..50}.jpeg -o images.zip`
Please relate any issues or contribute at https://github.com/vrmiguel/ouch")
        .author("Vinícius R. Miguel")
        .help_message("Displays this message and exits")
        .settings(&[
            clap::AppSettings::ColoredHelp,
            clap::AppSettings::ArgRequiredElseHelp,
        ])
        .arg(
            Arg::with_name("input")
                .required(true)
                .multiple(true)
                .long("input")
                .short("i")
                .help("The input files or directories.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                // --output/-o not required when output can be inferred from the input files
                .required(false)
                .multiple(false)
                .long("output")
                .short("o")
                .help("The output directory or compressed file.")
                .takes_value(true),
        )
}

pub fn get_matches() -> clap::ArgMatches<'static> {
    clap_app().get_matches()
}

// holy spaghetti code
impl TryFrom<clap::ArgMatches<'static>> for Command {
    type Error = error::Error;

    fn try_from(matches: clap::ArgMatches<'static>) -> error::OuchResult<Command> {
        let process_decompressible_input = |input_files: Values| {
            let input_files =
                input_files.map(|filename| (filename, Extension::new(filename)));

            for file in input_files.clone() {
                if let (file, Err(_)) = file {
                    return Err(error::Error::InputsMustHaveBeenDecompressible(file.into()));
                }
            }

            Ok(input_files
                .map(|(filename, extension)| (PathBuf::from(filename), extension.unwrap()))
                .map(File::from)
                .collect::<Vec<_>>())
        };

        // Possibilities:
        //   * Case 1: output not supplied, therefore try to infer output by checking if all input files are decompressible
        //   * Case 2: output supplied

        let output_was_supplied = matches.is_present("output");

        let input_files = matches.values_of("input").unwrap(); // Safe to unwrap since input is an obligatory argument

        if output_was_supplied {
            let output_file = matches.value_of("output").unwrap(); // Safe unwrap since we've established that output was supplied

            let output_file_extension = Extension::new(output_file);
            
            let output_is_compressible = output_file_extension.is_ok();
            if output_is_compressible {
                // The supplied output is compressible, so we'll compress our inputs to it

                // println!(
                //     "{}: trying to compress input files into '{}'",
                //     "info".yellow(),
                //     output_file
                // );

                let input_files = input_files.map(PathBuf::from).collect();

                return Ok(Command {
                    kind: CommandKind::Compression(input_files),
                    output: Some(File {
                        path: output_file.into(),
                        contents_in_memory: None,
                        extension: Some(output_file_extension.unwrap())
                    }),
                });

            } else {
                // Output not supplied
                // Checking if input files are decompressible

                let input_files = process_decompressible_input(input_files)?;

                return Ok(Command {
                    kind: CommandKind::Decompression(input_files),
                    output: Some(File {
                        path: output_file.into(),
                        contents_in_memory: None,
                        extension: None
                    })
                });
            }
        } else {
            // else: output file not supplied
            // Case 1: all input files are decompressible
            // Case 2: error
            let input_files = process_decompressible_input(input_files)?;

            return Ok(Command {
                kind: CommandKind::Decompression(input_files),
                output: None,
            });
        }
    }
}
