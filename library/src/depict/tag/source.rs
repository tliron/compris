use super::super::super::annotate::*;

use {depiction::*, std::io};

/// Source tag for a [Depict](Depict).
pub fn source<AnnotatedFieldsT, WriteT>(
    annotated_fields: &AnnotatedFieldsT,
    field_name: &str,
    writer: &mut WriteT,
    context: &DepictionContext,
) -> io::Result<()>
where
    AnnotatedFieldsT: AnnotatedStruct,
    WriteT: io::Write,
{
    if let Some(annotations) = annotated_fields.field_annotations(field_name)
        && let Some(source) = &annotations.source
    {
        context.separate(writer)?;
        context.theme.write_delimiter(writer, DEPICT_LOCATION_PREFIX)?;
        context.theme.write_meta(writer, source)?;
    }

    Ok(())
}
