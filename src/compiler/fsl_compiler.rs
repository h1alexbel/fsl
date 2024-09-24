// The MIT License (MIT)
//
// Copyright (c) 2024 Aliaksei Bialiauski
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
use crate::parser::fsl_parser::FslParser;
use log::info;
use std::fs;
use std::path::Path;

/// FSL compiler.
/// ```
/// use fsl::compiler::fsl_compiler::Fslc;
/// let output = Fslc::program(String::from("me: @jeff\n +repo me/foo")).out();
/// ```
pub struct Fslc {
    /// Program to compile.
    pub program: String,
    /// Parser.
    pub parser: FslParser,
}

impl Fslc {
    /// New compiler for program.
    pub fn program(program: String) -> Fslc {
        Fslc {
            program,
            parser: FslParser {},
        }
    }

    /// New compiler for program in file.
    pub fn file(path: &Path) -> Fslc {
        Fslc {
            program: fs::read_to_string(path).unwrap_or_else(|_| {
                panic!("Failed to read path: {}", path.display())
            }),
            parser: FslParser {},
        }
    }

    /// Out.
    pub fn out(self) {
        info!("Done!");
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::fsl_compiler::Fslc;
    use anyhow::Result;
    use log::Level;
    use std::path::Path;
    extern crate testing_logger;

    #[test]
    fn compiles_program_as_string() -> Result<()> {
        testing_logger::setup();
        Fslc::program(String::from("me: @jeff +repo me/foo")).out();
        testing_logger::validate(|logs| {
            assert_eq!(logs.len(), 1);
            assert_eq!(logs[0].body, "Done!");
            assert_eq!(logs[0].level, Level::Info);
        });
        Ok(())
    }

    #[test]
    fn compiles_program_from_file() -> Result<()> {
        testing_logger::setup();
        Fslc::file(Path::new("resources/programs/me.fsl")).out();
        testing_logger::validate(|logs| {
            assert_eq!(logs.len(), 1);
            assert_eq!(logs[0].body, "Done!");
            assert_eq!(logs[0].level, Level::Info);
        });
        Ok(())
    }
}
