use {
    depiction::*,
    std::{fmt, io},
};

//
// Location
//

/// Location annotation.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Location {
    /// Index.
    ///
    /// Note that it can be a byte index *or* a rune index, depending on the implementation.
    pub index: Option<usize>,

    /// Row.
    pub row: Option<usize>,

    /// Column. Will be ignored if row is [None].
    pub column: Option<usize>,
}

impl Location {
    /// Constructor.
    pub fn new(index: Option<usize>, row: Option<usize>, column: Option<usize>) -> Self {
        Self { index, row, column }
    }

    /// Whether [Depict] will have output.
    pub fn has_debug(&self) -> bool {
        self.row.is_some() || self.index.is_some()
    }
}

impl Depict for Location {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        if let Some(row) = self.row {
            // Though our row and column start at 0, users usually expect them to start at 1

            context.theme.write_number(writer, row + 1)?;

            if let Some(column) = self.column {
                context.theme.write_delimiter(writer, DEPICT_COORDINATE_SEPARATOR)?;
                context.theme.write_number(writer, column + 1)?;
            }
        } else if let Some(index) = self.index {
            // We'll show the index only if there is no row/column

            context.theme.write_delimiter(writer, DEPICT_INDEX_START)?;
            context.theme.write_number(writer, index)?;
            context.theme.write_delimiter(writer, DEPICT_INDEX_END)?;
        }

        Ok(())
    }
}

impl fmt::Display for Location {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if let Some(row) = self.row {
            // Though our row and column start at 0, users usually expect them to start at 1

            write!(formatter, "{}", row + 1)?;

            if let Some(column) = self.column {
                write!(formatter, "{}{}", DEPICT_COORDINATE_SEPARATOR, column + 1)?;
            }
        } else if let Some(index) = self.index {
            // We'll show the index only if there is no row/column
            write!(formatter, "{}{}{}", DEPICT_INDEX_START, index, DEPICT_INDEX_END)?;
        }

        Ok(())
    }
}
