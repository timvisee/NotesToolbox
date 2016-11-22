use super::doc_type::DocType;

pub struct Parser {
    doc_type: &'static DocType
}

impl Parser {
    /// Constructor.
    ///
    /// The document type for this parser should be passed to the `doc_type` parameter.
    fn new(doc_type: &'static DocType) -> Parser {
        Parser {
            doc_type: doc_type
        }
    }

    /// Get the document type this parser is for
    fn get_type(&self) -> &'static DocType {
        self.doc_type
    }
}
