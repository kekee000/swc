#[cfg(feature = "serde-impl")]
use serde::{Deserialize, Serialize};
use swc_ecma_ast::EsVersion;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde-impl", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde-impl", serde(rename_all = "camelCase"))]
pub struct Config {
    /// The target runtime environment.
    ///
    /// This defaults to [EsVersion::latest] because it preserves input as much
    /// as possible.
    ///
    /// Note: This does not verifies if output is valid for the target runtime.
    /// e.g. `const foo = 1;` with [EsVersion::Es3] will emitted as `const foo =
    /// 1` without verification.
    /// This is because it's not a concern of the code generator.
    #[cfg_attr(feature = "serde-impl", serde(default = "EsVersion::latest"))]
    pub target: EsVersion,

    /// Forces the code generator to use only ascii characters.
    ///
    /// This is useful for environments that do not support unicode.
    #[cfg_attr(feature = "serde-impl", serde(default))]
    pub ascii_only: bool,

    #[cfg_attr(feature = "serde-impl", serde(default))]
    pub minify: bool,

    /// If true, the code generator will emit the lastest semicolon.
    ///
    /// Defaults to `false`.
    #[cfg_attr(feature = "serde-impl", serde(default))]
    pub omit_last_semi: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            target: EsVersion::latest(),
            minify: false,
            ascii_only: false,
            omit_last_semi: false,
        }
    }
}
