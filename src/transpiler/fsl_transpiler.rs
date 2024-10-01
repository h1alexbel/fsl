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
use crate::parser::fsl_parser::{FslParser, Rule};
use pest::Parser;
use std::fs;
use std::path::Path;

/// FSL transpiler.
/// ```
/// use fsl::transpiler::fsl_transpiler::Fslt;
/// let output = Fslt::program(String::from("me: @jeff\n +repo me/foo")).out();
/// ```
pub struct Fslt {
    /// Program to transpiler.
    pub program: String,
}

impl Fslt {
    /// New transpiler for program.
    pub fn program(program: String) -> Fslt {
        Fslt { program }
    }

    /// New transpiler for program in file.
    pub fn file(path: &Path) -> Fslt {
        Fslt {
            program: fs::read_to_string(path).unwrap_or_else(|_| {
                panic!("Failed to read path: {}", path.display())
            }),
        }
    }

    /// Out.
    pub fn out(&self) -> Vec<&str> {
        let mut ast = vec![];
        let pairs = FslParser::parse(Rule::program, &self.program)
            .expect("Failed to parse program");
        for pair in pairs {
            match pair.as_rule() {
                Rule::program => {
                    for inner in pair.into_inner() {
                        match inner.as_rule() {
                            Rule::me => {
                                for me in inner.into_inner() {
                                    if me.as_rule() == Rule::login {
                                        let login = me.as_str();
                                        ast.push(login);
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        ast
    }
}

#[cfg(test)]
mod tests {
    use crate::transpiler::fsl_transpiler::Fslt;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};
    use crate::sample_program::sample_program;

    #[test]
    fn transpiles_program_as_string() -> Result<()> {
        testing_logger::setup();
        let program = String::from("me: @jeff\n +repo me/foo > x");
        let fsl = Fslt::program(program);
        let ast = fsl.out();
        let first = ast.first().expect("Failed to get first value");
        assert_that!(first, is(equal_to("@jeff")));
        Ok(())
    }

    #[test]
    fn transpiles_program_from_file() -> Result<()> {
        let transpiler = Fslt::program(sample_program("plusrepo-plusbar.fsl"));
        let ast = transpiler.out();
        let first = ast.first().expect("Failed to get first value");
        assert_that!(first, is(equal_to("@jeff")));
        Ok(())
    }
}