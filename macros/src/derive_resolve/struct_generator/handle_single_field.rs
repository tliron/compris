use super::generator::*;

use {proc_macro2::*, quote::*};

impl StructGenerator {
    /// Generate single field handler.
    pub fn generate_handle_single_field(&self) -> TokenStream {
        if let Some(single_field) = &self.single_field {
            let field_name = &single_field.name;

            let (handle_annotations1, handle_annotations2) =
                if let Some(annotations_field_name) = &self.annotations_field {
                    let annotated_parameter = self.annotated_parameter();
                    let quoted_field_name = field_name.to_string().to_token_stream();
                    (
                        quote! {
                            let self_annotations = if #annotated_parameter::can_have_annotations()
                                && let ::std::option::Option::Some(annotations) =
                                ::compris::annotate::Annotated::annotations(&self)
                            {
                                ::std::option::Option::Some(annotations.clone())
                            } else {
                                ::std::option::Option::None
                            };
                        },
                        quote! {
                            if let ::std::option::Option::Some(self_annotations) = self_annotations {
                                resolved.#annotations_field_name.insert(
                                    ::kutil::std::immutable::ByteString::from_static(#quoted_field_name),
                                    self_annotations.clone(),
                                );
                                resolved.#annotations_field_name.insert(
                                    ::kutil::std::immutable::ByteString::from_static(""),
                                    self_annotations,
                                );
                            }
                        },
                    )
                } else {
                    (Default::default(), Default::default())
                };

            quote! {
                #handle_annotations1
                if let ::std::option::Option::Some(value) =
                    ::compris::resolve::Resolve::resolve_with_errors(self, errors)?
                {
                    resolved.#field_name = value;
                    #handle_annotations2
                }

                return ::compris::resolve::ResolveResult::Ok(
                    ::std::option::Option::Some(resolved)
                );
            }
        } else {
            quote! {
                ::kutil::std::error::ErrorRecipient::give_error(
                    errors,
                    ::compris::normal::IncompatibleVariantTypeError::new_from(
                        &self,
                        &["map"],
                    ).into(),
                )?;

                return ::compris::resolve::ResolveResult::Ok(
                    ::std::option::Option::Some(resolved)
                );
            }
        }
    }
}
