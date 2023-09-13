macro_rules! impl_for_enum {
    (
        enum $enum_name:ident {
            $(
            $variant_name:ident($variant_ty:ty),
            )*
        }
    ) => {
        impl Foo for $enum_name {

            fn static_print_fields() {
                $(
                println!("Register {} for {}", stringify!{$variant_name}, stringify!{$enum_name});
                // <$variant_ty>::static_print_fields();
                )*
            }

            fn print_fields(&self) {
            }
        }

    };
}

macro_rules! impl_for_struct {
    (
        struct $struct_name:ident {
            $(
                $field_name:ident: $field_ty:ty,
            )*

            $(
                #[dissect(vec)]
                $vec_name:ident: Vec<$vec_ty:ty>,
            )*

            $(
                #[dissect(option)]
                $option_name:ident: Option<$option_ty:ty>,
            )*

            $(
                #[dissect(enum)]
                $enum_name:ident: $enum_ty:ty,
            )*
        }
    ) => {
        impl IntoHFMap for $struct_name {
            fn into_hf_map(self, prefix: &str) -> HeaderFieldMap {
                todo!()
            }
        }

        impl GenerateHFMap for InitAck {
            fn span() -> Span<Self> {
                todo!()
            }

            fn generate_hf_map(prefix: &str) -> HeaderFieldMap {
                let mut hf_map = HeaderFieldMap::new()

                $(
                    .add(prefix, stringify!{$field_name}, stringify!{$field_ty})
                )*

                $(
                    .add(prefix, stringify!{$vec_name}, stringify!{$vec_ty})
                )*

                $(
                    .add(prefix, stringify!{$enum_name}, stringify!{$enum_ty})
                )*
                ;

                hf_map
            }
        }
    };
}

pub(crate) use impl_for_struct;
pub(crate) use impl_for_enum;
