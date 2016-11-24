use super::doc_type::DocType;
use super::formatter_set::FormatterSet;

/// Document formatter.
/// This formatter formats and document from it's document elements to the preferred document type.
///
/// The proper document type and formatter set should be set to ensure the desired formatter is used.
pub struct Formatter {
    doc_type: DocType,
    formatter_set: FormatterSet
}

impl Formatter {
    /// Constructor.
    ///
    /// The document type for this formatter should be passed to the `doc_type` parameter.
    pub fn new(doc_type: DocType, formatter_set: FormatterSet) -> Formatter {
        Formatter {
            doc_type: doc_type,
            formatter_set: formatter_set
        }
    }

    /// Get the document type this formatter is for
    fn get_type(&self) -> &DocType {
        &self.doc_type
    }

    // Get the formatter set
    fn get_formatter_set(&self) -> &FormatterSet {
        &self.formatter_set
    }
}
