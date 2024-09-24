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
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};
    use parameterized::parameterized;
    use pest::Parser;

    #[test]
    // @todo #5:35min Supply FSL programs from input files.
    //  Instead of supplying FSL programs as strings here, we should read them
    //  from `resources/snippets/`.
    fn parses_program() -> Result<()> {
        let parsed = FslParser::parse(
            Rule::program,
            "me: @jeff\n+repo me/foo > x\n+repo me/bar > y\n",
        )
        .expect("Failed to parse FSL syntax")
        .next()
        .expect("Failed to get pair");
        let pairs = parsed.into_inner().as_str();
        assert_that!(
            pairs,
            is(equal_to("me: @jeff\n+repo me/foo > x\n+repo me/bar > y"))
        );
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
}
