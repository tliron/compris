use super::{
    super::{annotate::*, normal::*, *},
    errors::*,
    serializer::*,
};

use {
    depiction::*,
    std::{fs::*, io, path},
};

//
// RepresentationWriter
//

/// Either serializes (using [Serializer]) or writes the [Depict].
pub struct RepresentationWriter {
    /// Optional format. When [None] will write a debug depiction.
    format: Option<Format>,

    /// Pretty output (for YAML, JSON, and XML).
    pretty: bool,

    /// Base64 output (for CBOR and MessagePack).
    base64: bool,
}

impl RepresentationWriter {
    /// Constructor.
    pub fn new(format: Option<Format>, pretty: bool, base64: bool) -> Self {
        Self { format, pretty, base64 }
    }

    /// Write to file or stdout.
    pub fn write_to_file_or_stdout<PathT, AnnotatedT>(
        &self,
        variant: &Variant<AnnotatedT>,
        quiet: bool,
        quiet_empty: bool,
        trace: bool,
        path: Option<PathT>,
    ) -> Result<(), SerializeError>
    where
        PathT: AsRef<path::Path>,
        AnnotatedT: Annotated + Clone + Default,
    {
        let format = if trace && let Some(format) = self.format { format.as_str() } else { "debug depiction" };

        if let Some(path) = path {
            let path = path.as_ref();
            if trace {
                tracing::info!("writing {} to file: {}", format, path.display());
            }

            self.write(variant, &mut io::BufWriter::new(File::create(path)?))?;
        } else if quiet {
            if quiet_empty {
                if trace {
                    tracing::info!("writing {} to empty", format);
                }

                self.write(variant, &mut io::empty())?;
            }
        } else if self.is_binary() {
            if trace {
                tracing::info!("writing {} to stdout (raw)", format);
            }

            self.write(variant, &mut io::stdout())?;
        } else {
            if trace {
                tracing::info!("writing {} to stdout", format);
            }

            self.write(variant, &mut anstream::stdout())?;
        }

        Ok(())
    }

    /// Write.
    pub fn write<AnnotatedT, WriteT>(
        &self,
        variant: &Variant<AnnotatedT>,
        mut writer: WriteT,
    ) -> Result<(), SerializeError>
    where
        AnnotatedT: Annotated + Clone + Default,
        WriteT: io::Write,
    {
        match self.format {
            Some(format) => Serializer::new(format)
                .with_pretty(self.pretty)
                .with_base64(self.base64)
                .write_modal(variant, &mut writer),

            None => Ok(if self.pretty {
                variant.write_default_depiction(&mut writer)
            } else {
                variant.write_plain_depiction(&mut writer)
            }?),
        }
    }

    // Don't use anstream::stdout with raw binary.
    fn is_binary(&self) -> bool {
        if let Some(format) = self.format
            && format.is_binary()
            && !self.base64
        {
            true
        } else {
            false
        }
    }
}
