trait DocumentType {
    /// Get the type of the document.
    fn get_name() -> &'static str;
}