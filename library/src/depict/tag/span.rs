use super::super::super::annotate::*;

use {depiction::*, std::io};

/// Location tag for [Depict](depiction::Depict).
pub fn span<AnnotatedFieldsT, WriteT>(
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
        && let Some(span) = &annotations.span
    {
        context.separate(writer)?;
        context.theme.write_delimiter(writer, DEPICT_ANNOTATIONS_PREFIX)?;
        span.depict(writer, context)?;
    }

    Ok(())
}
