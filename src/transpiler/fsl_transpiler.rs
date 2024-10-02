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
use log::info;
use pest::Parser;
use serde_json::{Map, Value};
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
    pub fn out(&self) -> Value {
        let mut ast = Map::new();
        let pairs = FslParser::parse(Rule::program, &self.program)
            .expect("Failed to parse program");
        let mut cast = Vec::new();
        for pair in pairs {
            if pair.as_rule() == Rule::program {
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::me => {
                            for me in inner.into_inner() {
                                if me.as_rule() == Rule::login {
                                    ast.insert(
                                        String::from("login"),
                                        Value::String(String::from(
                                            me.as_str(),
                                        )),
                                    );
                                }
                            }
                        }
                        Rule::command => {
                            let mut cob = Map::new();
                            for command in inner.into_inner() {
                                if command.as_rule() == Rule::object {
                                    for object in command.into_inner() {
                                        if object.as_rule() == Rule::oid {
                                            cob.insert(
                                                String::from("oid"),
                                                Value::String(String::from(
                                                    object.as_str(),
                                                )),
                                            );
                                        } else if object.as_rule()
                                            == Rule::attributes
                                        {
                                            cob.insert(
                                                String::from("attrs"),
                                                Value::String(String::from(
                                                    object.as_str(),
                                                )),
                                            );
                                        } else if object.as_rule() == Rule::new
                                        {
                                            for new in object.into_inner() {
                                                if new.as_rule()
                                                    == Rule::reference
                                                {
                                                    cob.insert(
                                                        String::from("ref"),
                                                        Value::String(
                                                            String::from(
                                                                new.as_str(),
                                                            ),
                                                        ),
                                                    );
                                                }
                                            }
                                        } else if object.as_rule()
                                            == Rule::application
                                        {
                                            for application in
                                                object.into_inner()
                                            {
                                                if application.as_rule()
                                                    == Rule::reference
                                                {
                                                    cob.insert(
                                                        String::from(
                                                            "application",
                                                        ),
                                                        Value::String(
                                                            String::from(
                                                                application
                                                                    .as_str(),
                                                            ),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            cast.push(Value::Object(cob));
                        }
                        _ => {}
                    }
                }
            }
        }
        ast.insert(String::from("commands"), Value::Array(cast));
        info!(
            "Transpiled {} line(s) of FSL code to JSON!",
            self.program.lines().count()
        );
        Value::Object(ast)
    }
}

#[cfg(test)]
mod tests {
    use crate::sample_program::sample_program;
    use crate::transpiler::fsl_transpiler::Fslt;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};
    use log::Level;
    use std::path::Path;

    #[test]
    fn transpiles_program_as_string() -> Result<()> {
        testing_logger::setup();
        let program = String::from("me: @jeff\n");
        let fsl = Fslt::program(program);
        let ast = fsl.out();
        testing_logger::validate(|logs| {
            assert_eq!(logs.len(), 1);
            assert_eq!(
                logs[0].body,
                "Transpiled 1 line(s) of FSL code to JSON!"
            );
            assert_eq!(logs[0].level, Level::Info);
        });
        assert_that!(ast["login"].as_str(), is(equal_to(Some("@jeff"))));
        Ok(())
    }

    #[test]
    fn transpiles_program_from_file() -> Result<()> {
        let transpiler = Fslt::file(Path::new("resources/programs/me.fsl"));
        let ast = transpiler.out();
        assert_that!(ast["login"].as_str(), is(equal_to(Some("@jeff"))));
        Ok(())
    }

    #[test]
    fn transpiles_full_program() -> Result<()> {
        let transpiler = Fslt::program(sample_program("plusfoo-plusbar.fsl"));
        let ast = transpiler.out();
        let commands =
            ast["commands"].as_array().expect("Failed to get commands");
        let first = commands.first().expect("Failed to get first command");
        let second = commands.get(1).expect("Failed to get second command");
        assert_that!(commands.len(), is(equal_to(2)));
        assert_that!(ast["login"].as_str(), is(equal_to(Some("@jeff"))));
        assert_that!(first["oid"].as_str(), is(equal_to(Some("repo"))));
        assert_that!(first["attrs"].as_str(), is(equal_to(Some("me/foo"))));
        assert_that!(first["ref"].as_str(), is(equal_to(Some("x"))));
        assert_that!(second["oid"].as_str(), is(equal_to(Some("repo"))));
        assert_that!(second["attrs"].as_str(), is(equal_to(Some("me/bar"))));
        assert_that!(second["ref"].as_str(), is(equal_to(Some("y"))));
        Ok(())
    }

    #[test]
    fn transpiles_program_with_application() -> Result<()> {
        let transpiler = Fslt::program(sample_program("application.fsl"));
        let ast = transpiler.out();
        let commands =
            ast["commands"].as_array().expect("Failed to get commands");
        let command = commands
            .iter()
            .find(|c| c["oid"].as_str() == Some("issue"))
            .expect("Failed to find command");
        assert_that!(command["application"].as_str(), is(equal_to(Some("x"))));
        Ok(())
    }
}
