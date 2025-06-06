use ir::IR;

/// Trait for converting TypeScript code into IR structure.
///
/// This trait is the entry point for converting TypeScript source code into
/// the corresponding IR structure.
///
/// # Example
///
/// ```
/// use ir::IR;
/// use ts2ir::IRFromTypeScript;
///
/// let ts_code = r#"class Cat {
///     energy: number;
///     breath: number;
/// }"#;
/// let ir = IR::from_typescript(ts_code);
/// ```
pub trait IRFromTypeScript {
    /// Converts TypeScript source code to an IR structure.
    ///
    /// # Parameters
    ///
    /// * `input` - The TypeScript source code to convert
    ///
    /// # Returns
    ///
    /// An `IR` structure containing the converted elements
    fn from_typescript(input: &str) -> IR;
}
