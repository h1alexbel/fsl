use crate::transpiler::fsl_transpiler::Fslt;
use log::info;
use serde_json::{json, Value};
use std::collections::HashSet;

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
/// AST, decorated with errors.
pub struct ErrAst {
    /// Base AST.
    pub base: Fslt,
}

impl ErrAst {
    /// New.
    pub fn new(base: Fslt) -> ErrAst {
        ErrAst { base }
    }

    /// Decorate.
    pub fn decorate(self) -> Value {
        let ast = self.base.out();
        let mut errors = Vec::new();
        let commands = ast
            .get("program")
            .and_then(|p| p.get("commands"))
            .and_then(|c| c.as_array())
            .expect("failed to get commands");
        let refs: Vec<&Value> = commands
            .iter()
            .filter_map(|c| {
                if let Some(r) = c.get("ref") {
                    Some(r)
                } else {
                    None
                }
            })
            .collect();
        let mut seen = HashSet::new();
        let mut duplicates = Vec::new();
        for r in refs {
            if !seen.insert(r.as_str().expect("failed to get ref")) {
                duplicates.push(r.as_str().expect("failed to get ref"));
            }
        }
        if !duplicates.is_empty() {
            errors.push(format!("Duplicate refs: {:?}", duplicates));
        }
        json!(
            {
                "program": ast["program"],
                "errors": errors
            }
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::sample_program::sample_program;
    use crate::transpiler::err_ast::ErrAst;
    use crate::transpiler::fsl_transpiler::Fslt;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[test]
    fn adds_error_for_duplicate_refs() -> Result<()> {
        let ast = ErrAst::new(Fslt::program(sample_program(
            "errors/duplicate-refs.fsl",
        )))
        .decorate();
        assert_that!(
            ast["errors"]
                .as_array()
                .expect("failed to get errors")
                .is_empty(),
            is(equal_to(false))
        );
        Ok(())
    }
}
