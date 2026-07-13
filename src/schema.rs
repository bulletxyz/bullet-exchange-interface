use std::collections::HashMap;

// reexport to make it useable without thinking about the right dependencies
pub use sov_universal_wallet::schema::Schema;
use sov_universal_wallet::schema::{IndexLinking, Link};
use sov_universal_wallet::ty::{NamedField, Ty, UnnamedField};

#[derive(serde::Deserialize)]
pub struct SchemaFile {
    pub chain_hash: String,
    pub schema: Schema,
}

/// Trim a given schema by only returning the types for the necessary variants.
pub fn trim(input: &Schema, filter_variants: &dyn Fn(&str, &str) -> bool) -> Vec<Ty<IndexLinking>> {
    let types = input.types();
    let mut res = vec![];
    let mut map = HashMap::new();
    trim_internal(0, types, filter_variants, &mut map, &mut res);
    res
}

fn trim_internal(
    index: usize,
    types: &[Ty<IndexLinking>],
    filter_variants: &dyn Fn(&str, &str) -> bool,
    map: &mut HashMap<usize, usize>,
    res: &mut Vec<Ty<IndexLinking>>,
) {
    if map.contains_key(&index) {
        return;
    }
    let ofs = res.len();
    map.insert(index, ofs);
    let mut elem = types.get(index).expect("type not found").clone();
    res.push(elem.clone());

    match elem {
        Ty::Enum(ref mut value) => {
            let type_name = value.type_name.as_str();
            value
                .variants
                .retain(|x| filter_variants(type_name, x.name.as_str()));
            for v in &mut value.variants {
                if let Some(x) = &mut v.value {
                    replace_link(x, types, filter_variants, map, res);
                }
            }
        }
        Ty::Tuple(ref mut value) => {
            for f in &mut value.fields {
                let UnnamedField::<IndexLinking> {
                    value,
                    silent: _,
                    doc: _,
                } = f;
                replace_link(value, types, filter_variants, map, res);
            }
        }
        Ty::Struct(ref mut value) => {
            for f in &mut value.fields {
                let NamedField::<IndexLinking> {
                    value,
                    silent: _,
                    doc: _,
                    display_name: _,
                } = f;
                replace_link(value, types, filter_variants, map, res);
            }
        }
        Ty::Option { ref mut value }
        | Ty::Vec { ref mut value }
        | Ty::Array {
            ref mut value,
            len: _,
        } => {
            replace_link(value, types, filter_variants, map, res);
        }
        Ty::Map {
            ref mut key,
            ref mut value,
        } => {
            replace_link(key, types, filter_variants, map, res);
            replace_link(value, types, filter_variants, map, res);
        }
        Ty::String
        | Ty::Boolean
        | Ty::Float32
        | Ty::Float64
        | Ty::ByteVec { .. }
        | Ty::Integer { .. }
        | Ty::ByteArray { .. }
        | Ty::Skip { .. } => {}
    }
    // overwrite the changed element
    res[ofs] = elem;
}

fn replace_link(
    v: &mut Link,
    types: &[Ty<IndexLinking>],
    filter_variants: &dyn Fn(&str, &str) -> bool,
    map: &mut HashMap<usize, usize>,
    res: &mut Vec<Ty<IndexLinking>>,
) {
    match v {
        Link::ByIndex(index) => {
            let old = *index;
            *index = map.get(&old).copied().unwrap_or(res.len());
            if *index == res.len() {
                trim_internal(old, types, filter_variants, map, res);
            }
        }
        Link::Immediate(_) => {}
        Link::Placeholder | Link::IndexedPlaceholder(_) => {
            panic!("constructed schemas must not contain placeholders");
        }
    }
}
