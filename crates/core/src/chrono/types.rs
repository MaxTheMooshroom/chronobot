
use std::ops::RangeInclusive;

use crate::consts::*;

pub struct Agent {
    mutant_classes: [MutationInstance; NUM_AGENT_MUTANT_CLASSES],
    extra_mutations: Vec<MutationInstance>,
}

#[derive(Clone, Debug)]
pub struct MutationType {
    name: String,
    description: String,
    variants: (),
}

#[derive(Clone, Debug)]
pub struct MutationVariant {
    kind: String,
    description: String,
    values: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct MutationInstance {
    _type: &'static MutationType,
    variant: &'static str,
}

#[derive(Clone, Debug)]
pub struct MutantClass {
    name:           String,
    description:    String,
    subclasses:     Vec<MutantSubclass>,
}

#[derive(Clone, Debug)]
pub struct MutantSubclass {
    name:               String,
    description:        String,

    variants:           Vec<MutantSubclassVariant>,
}

#[derive(Clone, Debug)]
pub struct MutantSubclassVariant {
    pub name:           String,
    pub description:    String,
}

#[derive(Clone, Debug)]
pub struct MutantClassInstance {
    pub _type:      &'static MutantClass,
    pub subclasses: [&'static MutantSubclass; NUM_MUTANT_SUBCLASSES],
}

#[derive(Clone, Debug)]
pub struct MutantSubclassInstance {
    pub _type:          &'static MutantSubclass,
    pub variant_index:  Option<usize>,
}

