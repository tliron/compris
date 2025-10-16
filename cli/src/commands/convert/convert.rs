use super::super::{super::errors::*, root::*};

use compris::{annotate::*, ser::*, *};

impl Root {
    /// Convert.
    pub fn convert(&self) -> Result<(), MainError> {
        let variant = self.read::<WithAnnotations>()?;

        RepresentationWriter::new(self.output_format(), !self.output_plain, self.output_base64)
            .write_to_file_or_stdout(&variant, self.quiet, true, true, self.output_path.as_ref())?;

        Ok(())
    }

    fn output_format(&self) -> Option<Format> {
        match &self.output_format {
            Some(output_format) => output_format.to_compris(),
            None => self.output_path.as_ref().and_then(Format::from_path),
        }
    }
}
