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
use crate::transpiler::errors::check::Check;
use serde_json::Value;

/// Invalid application ref check.
pub struct InvalidApplicationRef {}

impl Check for InvalidApplicationRef {
    fn inspect(&self, ast: &Value) -> Vec<String> {
        let mut errors = Vec::new();
        let commands = ast
            .get("program")
            .and_then(|p| p.get("commands"))
            .and_then(|c| c.as_array())
            .expect("failed to get commands");
        let refs: Vec<&Value> =
            commands.iter().filter_map(|c| c.get("ref")).collect();
        let applications: Vec<&Value> = commands
            .iter()
            .filter_map(|c| c.get("application"))
            .collect();
        applications.iter().for_each(|a| {
            if !refs.contains(a) {
                errors.push(format!(
                    "Invalid application -> {}; Ref `{}` does not exist ",
                    a, a
                ));
            }
        });
        errors
    }
}

#[cfg(test)]
mod tests {
    use crate::sample_program::sample_program;
    use crate::transpiler::errors::check::Check;
    use crate::transpiler::errors::invalid_application_ref::InvalidApplicationRef;
    use crate::transpiler::fsl_transpiler::Fslt;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[test]
    fn adds_errors_for_invalid_application_ref() -> Result<()> {
        let transpiler =
            Fslt::program(sample_program("errors/invalid-application-ref.fsl"));
        let errors = InvalidApplicationRef {}.inspect(&transpiler.out());
        assert_that!(errors.is_empty(), is(equal_to(false)));
        Ok(())
    }
}
