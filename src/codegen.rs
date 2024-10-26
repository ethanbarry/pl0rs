use crate::Token::{self, *};

// Appends code to a mutable buffer string.
fn out(code: &str, mut buf: &str) {
    todo!()
}

/// Print `const long name = ` to buf.
pub fn c_const(name: Token, mut buf: &str) {
    // Unwrap name here somehow...
    out("const long {name} =", buf)
}

/// Print final comment when execution is complete.
/// Includes version number.
pub fn c_end(mut buf: &str) {
    out(
        &format!(
            "\n/* PL/0 Compiler -- pl0rs v.{} */",
            crate::COMPILER_VERSION
        ),
        buf,
    )
}
