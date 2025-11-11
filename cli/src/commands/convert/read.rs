use super::super::root::*;

use {
    clap::*,
    compris::{annotate::*, normal::*, parse::Parser, *},
    problemo::{common::*, *},
    read_url::*,
    std::{
        io::{self, IsTerminal},
        path,
    },
};

impl Root {
    pub fn read<AnnotatedT>(&self) -> Result<Variant<AnnotatedT>, Problem>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let mut reader = self.get_reader()?;
        let input_format = self.input_format()?;

        Parser::new(input_format.clone())
            .with_try_integers(self.input_integers)
            .with_try_unsigned_integers(self.input_unsigned_integers)
            .with_allow_legacy_words(self.input_legacy)
            .with_allow_legacy_types(self.input_legacy)
            .with_base64(self.input_base64)
            .parse_reader(&mut reader)
    }

    fn get_reader(&self) -> Result<ReadRef, Problem> {
        Ok(match &self.input_path_or_url {
            Some(input_url) => {
                let url_context = UrlContext::new();

                #[cfg(feature = "file")]
                let base_urls = url_context.working_dir_url_vec()?;

                #[cfg(not(feature = "file"))]
                let base_urls = Vec::default();

                let context = url_context.with_base_urls(base_urls);
                let url = context.url_or_file_path(input_url)?;

                tracing::info!("reading from URL: {}", url);
                Box::new(io::BufReader::new(url.open()?))
            }

            None => {
                let stdin = io::stdin();
                if stdin.is_terminal() {
                    Root::command().print_help()?;
                    return Err(ExitError::success());
                }

                tracing::info!("reading from stdin");
                Box::new(stdin)
            }
        })
    }

    fn input_format(&self) -> Result<Format, UnknownFormatError> {
        match &self.input_format {
            Some(format) => {
                let format = format.to_compris();
                tracing::info!("forced input format: {}", format);
                return Ok(format);
            }

            None => {
                if let Some(input_path_or_url) = &self.input_path_or_url
                    && let Some(format) = Format::from_path(path::Path::new(input_path_or_url))
                {
                    tracing::info!("input format from URL extension: {}", format);
                    return Ok(format);
                }
            }
        };

        Err("cannot determine input format; specify it explicitly with --input-format/-F".into())
    }
}
