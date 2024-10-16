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
use pest_derive::Parser;

/// FSL Syntax Parser.
#[derive(Parser)]
#[grammar = "program.pest"]
pub struct FslParser {}

#[cfg(test)]
mod tests {
    use crate::parser::fsl_parser::{FslParser, Rule};
    use crate::sample_program::sample_program;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};
    use parameterized::parameterized;
    use pest::Parser;

    #[parameterized(
        program = {
            &sample_program("me.fsl"),
            &sample_program("plusfoo-plusbar.fsl")
        }
      )
    ]
    fn parses_program(program: &str) -> Result<()> {
        let parsed = FslParser::parse(Rule::program, program)
            .expect("Failed to parse FSL syntax")
            .next()
            .expect("Failed to get pair");
        let pairs = parsed.into_inner().as_str();
        let mut trimmed = program.to_string();
        trimmed.pop().expect("Failed to remove last character");
        assert_that!(String::from(pairs), is(equal_to(trimmed)));
        Ok(())
    }

    #[test]
    fn parses_me() -> Result<()> {
        let parse = FslParser::parse(Rule::me, "me: @jeff")
            .expect("Failed to parse FSL syntax");
        assert_that!(parse.as_str(), is(equal_to("me: @jeff")));
        Ok(())
    }

    #[parameterized(input = {"@jeff", "@x"})]
    fn parses_login(input: &str) -> Result<()> {
        let parsed = FslParser::parse(Rule::login, input)
            .expect("Failed to parse login");
        assert_that!(parsed.as_str(), is(equal_to(input)));
        Ok(())
    }

    #[should_panic(expected = "Failed to parse login")]
    #[parameterized(
        input = {
            "@_f",
            "abc",
            "@",
            "testing@",
            "@."
        }
    )]
    fn panics_on_invalid_login(input: &str) {
        FslParser::parse(Rule::login, input).expect("Failed to parse login");
    }

    #[test]
    fn parses_command() -> Result<()> {
        let parsed = FslParser::parse(Rule::command, "+repo me/foo > foo")
            .expect("Failed to parse FSL syntax");
        assert_that!(parsed.as_str(), is(equal_to("+repo me/foo > foo")));
        Ok(())
    }

    #[test]
    fn parses_object() -> Result<()> {
        let parsed = FslParser::parse(Rule::object, "repo me/foo > foo")
            .expect("Failed to parse FSL syntax");
        assert_that!(parsed.as_str(), is(equal_to("repo me/foo > foo")));
        Ok(())
    }

    #[test]
    fn parses_object_without_attributes() -> Result<()> {
        let parsed = FslParser::parse(Rule::object, "repo > foo")
            .expect("Failed to parse FSL syntax");
        assert_that!(parsed.as_str(), is(equal_to("repo > foo")));
        Ok(())
    }

    #[test]
    fn parses_new() -> Result<()> {
        let parsed = FslParser::parse(Rule::new, "> x")
            .expect("Failed to parse FSL syntax");
        assert_that!(parsed.as_str(), is(equal_to("> x")));
        Ok(())
    }

    #[parameterized(input = {"x", "xy", "foo", "test"})]
    fn parses_ref(input: &str) -> Result<()> {
        let parsed = FslParser::parse(Rule::reference, input)
            .expect("Failed to parse FSL syntax");
        assert_that!(parsed.as_str(), is(equal_to(input)));
        Ok(())
    }

    #[test]
    fn parses_application() -> Result<()> {
        let parsed = FslParser::parse(Rule::application, "-> x")
            .expect("Failed to parse application syntax");
        assert_that!(parsed.as_str(), is(equal_to("-> x")));
        Ok(())
    }

    #[test]
    fn parses_object_with_application() -> Result<()> {
        let parsed = FslParser::parse(Rule::object, "issue testing -> x")
            .expect("Failed to parse object syntax");
        assert_that!(parsed.as_str(), is(equal_to("issue testing -> x")));
        Ok(())
    }

    #[should_panic(expected = "Failed to parse reference")]
    #[parameterized(
        input = {
            "_",
            "_test",
            "@",
            "!",
            "!bar",
            ".",
            "/t",
            "X",
            "XYZ"
        }
    )]
    fn panics_on_invalid_ref(input: &str) {
        FslParser::parse(Rule::reference, input)
            .expect("Failed to parse reference");
    }

    #[parameterized(
        input = {
            "# test",
            "# UPPER CASE",
            "#  extra space",
            "# with extra words",
            "# with dot in the end.",
            "# test, and another test.",
            "#sticky works too.",
            "# it works THIS way TOO.",
            "# parses with tag inside #",
            "# parses with text after tag # this way",
            "# `special` 'characters' ```test```"
        }
    )]
    fn parses_comment(input: &str) -> Result<()> {
        let parsed = FslParser::parse(Rule::comment, input)
            .expect("failed to parse comment");
        assert_that!(parsed.as_str(), is(equal_to(input)));
        Ok(())
    }

    #[test]
    fn parses_program_with_comments() -> Result<()> {
        let program = &sample_program("with-comments.fsl");
        let pairs = FslParser::parse(Rule::program, program)
            .expect("failed to parse program with comments");
        assert_that!(pairs.as_str().len(), is(equal_to(143)));
        Ok(())
    }

    #[test]
    fn parses_program_with_license() -> Result<()> {
        let program = &sample_program("with-license.fsl");
        let pairs = FslParser::parse(Rule::program, program)
            .expect("failed to parse program with license");
        assert_that!(pairs.as_str().len(), is(equal_to(211)));
        Ok(())
    }
}
