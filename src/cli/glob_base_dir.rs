//! The source code in this file is Copyright (c) 2017 Gilad Naaman.
//!
//! Copied and modified as allowed per [`MIT license`], also listed below.
//!
//! Changes made:
//! * Now returns the builder instead of a glob walker, so it can be further adapted to one's wishes.
//!
//!
//! ```text
//! Copyright (c) 2017 Gilad Naaman
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.
//! ```
//!
//! [`MIT license`]: https://docs.rs/crate/globwalk/0.8.0/source/LICENSE

use globwalk::GlobWalkerBuilder;
use std::path::PathBuf;

/// Function to determine the base directory and (remaining) pattern.
///
/// This function contains a partial copy of the [`globwalker::glob`] function w
///
/// [`globwalker::glob`]: https://docs.rs/globwalk/0.8.0/src/globwalk/lib.rs.html#430
pub fn glob_builder_base<PAT: AsRef<str>>(pattern: PAT) -> GlobWalkerBuilder {
    let path_pattern: PathBuf = pattern.as_ref().into();
    if path_pattern.is_absolute() {
        // If the pattern is an absolute path, split it into the longest base and a pattern.
        let mut base = PathBuf::new();
        let mut pattern = PathBuf::new();
        let mut globbing = false;

        // All `to_str().unwrap()` calls should be valid since the input is a string.
        for c in path_pattern.components() {
            let os = c.as_os_str().to_str().unwrap();
            for c in &["*", "{", "}"][..] {
                if os.contains(c) {
                    globbing = true;
                    break;
                }
            }

            if globbing {
                pattern.push(c);
            } else {
                base.push(c);
            }
        }

        let pat = pattern.to_str().unwrap();
        if cfg!(windows) {
            GlobWalkerBuilder::new(base.to_str().unwrap(), pat.replace("\\", "/"))
        } else {
            GlobWalkerBuilder::new(base.to_str().unwrap(), pat)
        }
    } else {
        // If the pattern is relative, start searching from the current directory.
        GlobWalkerBuilder::new(".", pattern)
    }
}
