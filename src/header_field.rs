use std::collections::HashMap;


#[derive(Debug)]
pub struct HeaderField {
    pub name: String,
    pub kind: FieldKind,
}

pub type HeaderFieldMap = HashMap<String, HeaderField>;

#[derive(Debug, Copy, Clone)]
pub enum FieldKind {
    Number,
    Text,
    Bytes,
    Branch,
}

pub trait IntoHFMap {
    fn into_hf_map(self, prefix: &str) -> HeaderFieldMap;
}

pub enum Span<T> {
    Enum(Vec<T>),
    Struct(T)
}

pub trait GenerateHFMap: Sized + IntoHFMap {

    fn span() -> Span<Self>;

    fn generate_hf_map(prefix: &str) -> HeaderFieldMap {
        match Self::span() {
            Span::Enum(branches) => {
                let mut hf_map = HeaderFieldMap::new();
                for item in branches {
                    hf_map.extend(item.into_hf_map(prefix.clone()));
                }
                hf_map
            }
            Span::Struct(st) => {
                st.into_hf_map(prefix)
            }
        }
    }
}
