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
use crate::transpiler::fsl_transpiler::Fslt;
use serde_json::Value;

/// AST, decorated with errors.
pub struct ErrAst {
    /// Base AST.
    pub base: Fslt,
    /// Checks.
    pub checks: Vec<Box<dyn Check>>,
}

impl ErrAst {
    /// Default.
    pub fn default(base: Fslt) -> ErrAst {
        ErrAst {
            base,
            checks: vec![],
        }
    }

    /// New.
    pub fn new(base: Fslt, checks: Vec<Box<dyn Check>>) -> ErrAst {
        ErrAst { base, checks }
    }

    /// Decorate.
    pub fn decorate(self) -> Value {
        let ast = self.base.out();

        // checks.iter().forEach(|c| c.decorate());
        Value::String(String::from(""))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn adds_error_for_duplicate_refs() -> Result<()> {
        Ok(())
    }
}
