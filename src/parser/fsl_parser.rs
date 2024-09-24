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
    use pest::Parser;

    #[test]
    fn creates_parser() -> Result<()> {
        let parse = FslParser::parse(Rule::me, "me: @jeff")
            .expect("Failed to parse FSL syntax");
        assert_that!(parse.as_str(), is(equal_to("me: @jeff")));
        Ok(())
    }

    #[test]
    fn parses_command() -> Result<()> {
        let parsed = FslParser::parse(Rule::command, "+repo me/foo > foo")
            .expect("Failed to parse FSL syntax");
        assert_that!(parsed.as_str(), is(equal_to("+repo me/foo > foo")));
        Ok(())
    }
}
