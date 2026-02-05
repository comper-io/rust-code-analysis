//! rust-code-analysis is a library to analyze and extract information
//! from source codes written in many different programming languages.
//!
//! You can find the source code of this software on
//! <a href="https://github.com/mozilla/rust-code-analysis/" target="_blank">GitHub</a>,
//! while issues and feature requests can be posted on the respective
//! <a href="https://github.com/mozilla/rust-code-analysis/issues/" target="_blank">GitHub Issue Tracker</a>.
//!
//! ## Supported Languages
//!
//! - C++
//! - C#
//! - CSS
//! - Go
//! - HTML
//! - Java
//! - JavaScript
//! - The JavaScript used in Firefox internal
//! - Python
//! - Rust
//! - Typescript
//!
//! ## Supported Metrics
//!
//! - CC: it calculates the code complexity examining the
//!   control flow of a program.
//! - SLOC: it counts the number of lines in a source file.
//! - PLOC: it counts the number of physical lines (instructions)
//!   contained in a source file.
//! - LLOC: it counts the number of logical lines (statements)
//!   contained in a source file.
//! - CLOC: it counts the number of comments in a source file.
//! - BLANK: it counts the number of blank lines in a source file.
//! - HALSTEAD: it is a suite that provides a series of information,
//!   such as the effort required to maintain the analyzed code,
//!   the size in bits to store the program, the difficulty to understand
//!   the code, an estimate of the number of bugs present in the codebase,
//!   and an estimate of the time needed to implement the software.
//! - MI: it is a suite that allows to evaluate the maintainability
//!   of a software.
//! - NOM: it counts the number of functions and closures
//!   in a file/trait/class.
//! - NEXITS: it counts the number of possible exit points
//!   from a method/function.
//! - NARGS: it counts the number of arguments of a function/method.

#![allow(clippy::upper_case_acronyms)]

mod c_langs_macros;
mod c_macro;
mod getter;
mod macros;

mod alterator;
pub use alterator::*;

mod node;
pub use crate::node::*;

mod metrics;
pub use metrics::*;

mod languages;
pub(crate) use languages::*;

mod checker;
pub(crate) use checker::*;

mod output;
pub use output::*;

mod spaces;
pub use crate::spaces::*;

mod ops;
pub use crate::ops::*;

mod find;
pub use crate::find::*;

mod function;
pub use crate::function::*;

mod ast;
pub use crate::ast::*;

mod count;
pub use crate::count::*;

mod preproc;
pub use crate::preproc::*;

mod vue_extract;
pub use crate::vue_extract::*;

mod langs;
pub use crate::langs::*;

mod tools;
pub use crate::tools::*;

mod concurrent_files;
pub use crate::concurrent_files::*;

mod traits;
pub use crate::traits::*;

mod parser;
pub use crate::parser::*;

mod comment_rm;
pub use crate::comment_rm::*;

/// Compute metrics for a Vue Single File Component by analyzing all sections
/// and combining the LoC metrics.
pub fn get_vue_metrics(source: Vec<u8>, path: &std::path::Path) -> Option<FuncSpace> {
    use std::path::PathBuf;

    let sections = extract_vue_sections(&source);

    // Create a combined metrics space
    let mut combined_space = FuncSpace {
        name: path.to_str().map(|s| s.to_string()),
        start_line: 1,
        end_line: source.iter().filter(|&&b| b == b'\n').count() + 1,
        kind: SpaceKind::Unit,
        spaces: Vec::new(),
        metrics: CodeMetrics::default(),
    };

    // Analyze template section as HTML
    if let Some(template_section) = sections.template {
        if let Some(template_metrics) = get_function_spaces(
            &LANG::Html,
            template_section.content,
            &PathBuf::from("template.html"),
            None,
        ) {
            // Merge LoC metrics from template
            combined_space
                .metrics
                .loc
                .merge(&template_metrics.metrics.loc);
        }
    }

    // Analyze script section as JavaScript or TypeScript
    if let Some(script_section) = sections.script {
        let script_lang = match script_section.lang.as_deref() {
            Some("ts") | Some("typescript") => &LANG::Typescript,
            _ => &LANG::Mozjs, // Default to JavaScript
        };

        if let Some(script_metrics) = get_function_spaces(
            script_lang,
            script_section.content,
            &PathBuf::from("script.js"),
            None,
        ) {
            // Merge all metrics from script (including complexity)
            combined_space
                .metrics
                .loc
                .merge(&script_metrics.metrics.loc);
            combined_space
                .metrics
                .cyclomatic
                .merge(&script_metrics.metrics.cyclomatic);
            combined_space
                .metrics
                .cognitive
                .merge(&script_metrics.metrics.cognitive);
            combined_space
                .metrics
                .halstead
                .merge(&script_metrics.metrics.halstead);
            combined_space
                .metrics
                .nom
                .merge(&script_metrics.metrics.nom);
            combined_space.metrics.mi.merge(&script_metrics.metrics.mi);
            combined_space
                .metrics
                .nargs
                .merge(&script_metrics.metrics.nargs);
            combined_space
                .metrics
                .nexits
                .merge(&script_metrics.metrics.nexits);
            combined_space
                .metrics
                .abc
                .merge(&script_metrics.metrics.abc);

            // Add script functions as subspaces
            combined_space.spaces.extend(script_metrics.spaces);
        }
    }

    // Analyze style section as CSS
    if let Some(style_section) = sections.style {
        // Only analyze plain CSS, skip SCSS/Less for now
        if style_section.lang.is_none() || style_section.lang.as_deref() == Some("css") {
            if let Some(style_metrics) = get_function_spaces(
                &LANG::Css,
                style_section.content,
                &PathBuf::from("style.css"),
                None,
            ) {
                // Merge LoC metrics from style
                combined_space.metrics.loc.merge(&style_metrics.metrics.loc);
            }
        }
    }

    Some(combined_space)
}
