use std::collections::{HashMap, HashSet};

// reexport to make it useable without thinking about the right dependencies
pub use sov_universal_wallet::schema::Schema;
use sov_universal_wallet::schema::{IndexLinking, Link};
use sov_universal_wallet::ty::Ty;

#[derive(serde::Deserialize)]
pub struct SchemaFile {
    pub chain_hash: String,
    pub schema: Schema,
}

/// Trim a schema to the variants selected by `filter_variants`.
pub fn trim(input: &Schema, filter_variants: &dyn Fn(&str, &str) -> bool) -> Vec<Ty<IndexLinking>> {
    trim_matching_variants(input, input, filter_variants).0
}

/// Trim two schemas to the variants that must be compatible.
///
/// Once an enum has a selected variant in either schema, it must have at least
/// one selected variant in common. Additional one-sided variants are ignored
/// unless the other schema uses the same discriminant for a different variant.
/// Those collisions are retained so the returned schemas compare unequal
/// instead of decoding one action as another.
pub fn trim_matching_variants(
    left: &Schema,
    right: &Schema,
    filter_variants: &dyn Fn(&str, &str) -> bool,
) -> (Vec<Ty<IndexLinking>>, Vec<Ty<IndexLinking>>) {
    Trimmer::new(left, right, filter_variants).trim()
}

struct Trimmer<'a> {
    left_types: &'a [Ty<IndexLinking>],
    right_types: &'a [Ty<IndexLinking>],
    filter_variants: &'a dyn Fn(&str, &str) -> bool,
    visited: HashMap<(usize, usize), usize>,
    left_result: Vec<Ty<IndexLinking>>,
    right_result: Vec<Ty<IndexLinking>>,
}

impl<'a> Trimmer<'a> {
    fn new(
        left: &'a Schema,
        right: &'a Schema,
        filter_variants: &'a dyn Fn(&str, &str) -> bool,
    ) -> Self {
        Self {
            left_types: left.types(),
            right_types: right.types(),
            filter_variants,
            visited: HashMap::new(),
            left_result: vec![],
            right_result: vec![],
        }
    }

    fn trim(mut self) -> (Vec<Ty<IndexLinking>>, Vec<Ty<IndexLinking>>) {
        self.visit(0, 0);
        (self.left_result, self.right_result)
    }

    fn visit(&mut self, left_index: usize, right_index: usize) -> usize {
        if let Some(index) = self.visited.get(&(left_index, right_index)) {
            return *index;
        }

        let index = self.left_result.len();
        self.visited.insert((left_index, right_index), index);
        let mut left = self
            .left_types
            .get(left_index)
            .expect("left type not found")
            .clone();
        let mut right = self
            .right_types
            .get(right_index)
            .expect("right type not found")
            .clone();
        self.left_result.push(left.clone());
        self.right_result.push(right.clone());

        self.trim_pair(&mut left, &mut right);
        self.left_result[index] = left;
        self.right_result[index] = right;
        index
    }

    fn trim_pair(&mut self, left: &mut Ty<IndexLinking>, right: &mut Ty<IndexLinking>) {
        match (left, right) {
            (Ty::Enum(left), Ty::Enum(right)) if left.type_name == right.type_name => {
                let type_name = left.type_name.clone();
                let selected_left = left
                    .variants
                    .iter()
                    .filter(|variant| (self.filter_variants)(&type_name, &variant.name))
                    .map(|variant| variant.name.clone())
                    .collect::<HashSet<_>>();
                let selected_right = right
                    .variants
                    .iter()
                    .filter(|variant| (self.filter_variants)(&type_name, &variant.name))
                    .map(|variant| variant.name.clone())
                    .collect::<HashSet<_>>();
                let common = selected_left
                    .intersection(&selected_right)
                    .cloned()
                    .collect::<HashSet<_>>();
                let collisions = left
                    .variants
                    .iter()
                    .filter(|variant| selected_left.contains(&variant.name))
                    .filter(|variant| {
                        right.variants.iter().any(|counterpart| {
                            counterpart.discriminant == variant.discriminant
                                && counterpart.name != variant.name
                        })
                    })
                    .map(|variant| variant.discriminant)
                    .chain(
                        right
                            .variants
                            .iter()
                            .filter(|variant| selected_right.contains(&variant.name))
                            .filter(|variant| {
                                left.variants.iter().any(|counterpart| {
                                    counterpart.discriminant == variant.discriminant
                                        && counterpart.name != variant.name
                                })
                            })
                            .map(|variant| variant.discriminant),
                    )
                    .collect::<HashSet<_>>();
                let reject_empty_intersection =
                    common.is_empty() && (!selected_left.is_empty() || !selected_right.is_empty());

                left.variants.retain(|variant| {
                    common.contains(&variant.name)
                        || collisions.contains(&variant.discriminant)
                        || (reject_empty_intersection && selected_left.contains(&variant.name))
                });
                right.variants.retain(|variant| {
                    common.contains(&variant.name)
                        || collisions.contains(&variant.discriminant)
                        || (reject_empty_intersection && selected_right.contains(&variant.name))
                });

                for left_variant in &mut left.variants {
                    let Some(right_variant) = right.variants.iter_mut().find(|right_variant| {
                        right_variant.name == left_variant.name
                            || (collisions.contains(&left_variant.discriminant)
                                && right_variant.discriminant == left_variant.discriminant)
                    }) else {
                        continue;
                    };
                    if let (Some(left), Some(right)) =
                        (&mut left_variant.value, &mut right_variant.value)
                    {
                        self.replace_links(left, right);
                    }
                }
            }
            (Ty::Struct(left), Ty::Struct(right)) => {
                for (left, right) in left.fields.iter_mut().zip(&mut right.fields) {
                    self.replace_links(&mut left.value, &mut right.value);
                }
            }
            (Ty::Tuple(left), Ty::Tuple(right)) => {
                for (left, right) in left.fields.iter_mut().zip(&mut right.fields) {
                    self.replace_links(&mut left.value, &mut right.value);
                }
            }
            (Ty::Option { value: left }, Ty::Option { value: right })
            | (Ty::Vec { value: left }, Ty::Vec { value: right })
            | (Ty::Array { value: left, .. }, Ty::Array { value: right, .. }) => {
                self.replace_links(left, right);
            }
            (
                Ty::Map {
                    key: left_key,
                    value: left_value,
                },
                Ty::Map {
                    key: right_key,
                    value: right_value,
                },
            ) => {
                self.replace_links(left_key, right_key);
                self.replace_links(left_value, right_value);
            }
            _ => {}
        }
    }

    fn replace_links(&mut self, left: &mut Link, right: &mut Link) {
        assert!(
            !matches!(left, Link::Placeholder | Link::IndexedPlaceholder(_))
                && !matches!(right, Link::Placeholder | Link::IndexedPlaceholder(_)),
            "constructed schemas must not contain placeholders"
        );
        if let (Link::ByIndex(left_index), Link::ByIndex(right_index)) = (&*left, &*right) {
            let index = self.visit(*left_index, *right_index);
            *left = Link::ByIndex(index);
            *right = Link::ByIndex(index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod left {
        #[allow(dead_code)]
        #[derive(
            borsh::BorshDeserialize, borsh::BorshSerialize, sov_universal_wallet::UniversalWallet,
        )]
        #[borsh(use_discriminant = true)]
        #[repr(u8)]
        pub enum Message {
            Shared(u64) = 0,
            LeftOnly = 1,
        }
    }

    mod compatible {
        #[allow(dead_code)]
        #[derive(
            borsh::BorshDeserialize, borsh::BorshSerialize, sov_universal_wallet::UniversalWallet,
        )]
        #[borsh(use_discriminant = true)]
        #[repr(u8)]
        pub enum Message {
            Shared(u64) = 0,
            RightOnly = 2,
        }
    }

    mod collision {
        #[allow(dead_code)]
        #[derive(
            borsh::BorshDeserialize, borsh::BorshSerialize, sov_universal_wallet::UniversalWallet,
        )]
        #[borsh(use_discriminant = true)]
        #[repr(u8)]
        pub enum Message {
            Shared(u64) = 0,
            RightOnly = 1,
        }
    }

    mod disjoint {
        #[allow(dead_code)]
        #[derive(
            borsh::BorshDeserialize, borsh::BorshSerialize, sov_universal_wallet::UniversalWallet,
        )]
        #[borsh(use_discriminant = true)]
        #[repr(u8)]
        pub enum Message {
            RightOnly = 2,
        }
    }

    #[test]
    fn ignores_one_sided_variants_with_distinct_discriminants() {
        let left_schema = Schema::of_single_type::<left::Message>().unwrap();
        let right_schema = Schema::of_single_type::<compatible::Message>().unwrap();
        let keep_all = |_: &str, _: &str| true;

        assert_eq!(
            trim(&left_schema, &keep_all),
            trim_matching_variants(&left_schema, &left_schema, &keep_all).0
        );

        let (left, right) = trim_matching_variants(&left_schema, &right_schema, &keep_all);
        assert_eq!(left, right);

        let (left, right) = trim_matching_variants(&right_schema, &left_schema, &keep_all);
        assert_eq!(left, right);
    }

    #[test]
    fn checks_collisions_against_filtered_counterparts() {
        let left = Schema::of_single_type::<left::Message>().unwrap();
        let right = Schema::of_single_type::<collision::Message>().unwrap();
        let select_local = |_: &str, variant: &str| matches!(variant, "Shared" | "LeftOnly");

        let (left, right) = trim_matching_variants(&left, &right, &select_local);
        assert_ne!(left, right);
    }

    #[test]
    fn rejects_selected_enum_without_shared_variants() {
        let left = Schema::of_single_type::<left::Message>().unwrap();
        let right = Schema::of_single_type::<disjoint::Message>().unwrap();
        let select_local = |_: &str, variant: &str| variant == "LeftOnly";

        let (left, right) = trim_matching_variants(&left, &right, &select_local);
        assert_ne!(left, right);
    }
}
