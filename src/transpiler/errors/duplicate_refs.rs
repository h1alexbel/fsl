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
use std::collections::HashSet;

/// Duplicate refs check.
pub struct DuplicateRefs {
    /// AST.
    pub ast: Value,
}

impl DuplicateRefs {
    /// New.
    pub fn new(ast: Value) -> DuplicateRefs {
        DuplicateRefs { ast }
    }
}

impl Check for DuplicateRefs {
    fn decorate(self) -> Value {
        let mut errors = Vec::new();
        let commands = self
            .ast
            .get("program")
            .and_then(|p| p.get("commands"))
            .and_then(|c| c.as_array())
            .expect("failed to get commands");
        let refs: Vec<&Value> = commands
            .iter()
            .filter_map(|c| c.get("ref").map(|r| r))
            .collect();
        let mut seen = HashSet::new();
        let mut duplicates = Vec::new();
        for r in refs {
            if !seen.insert(r.as_str().expect("failed to get ref")) {
                duplicates.push(r.as_str().expect("failed to get ref"));
            }
        }
        if !duplicates.is_empty() {
            errors.push(Value::String(format!(
                "Duplicated refs: {:?}",
                duplicates
            )));
        }
        Value::Array(errors)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::sample_program::sample_program;
    use crate::transpiler::errors::check::Check;
    use crate::transpiler::errors::duplicate_refs::DuplicateRefs;
    use crate::transpiler::fsl_transpiler::Fslt;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[test]
    fn adds_error_for_duplicate_refs() -> Result<()> {
        let decorated = DuplicateRefs::new(
            Fslt::program(sample_program("errors/duplicate-refs.fsl")).out(),
        )
        .decorate();
        assert_that!(
            decorated
                .as_array()
                .expect("failed to get errors")
                .is_empty(),
            is(equal_to(false))
        );
        Ok(())
    }
}
