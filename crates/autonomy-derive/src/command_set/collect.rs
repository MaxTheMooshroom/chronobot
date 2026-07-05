use deluxe::ExtractAttributes;
use syn::{DeriveInput, Member, Path, Variant};

use std::collections::HashMap;

#[derive(ExtractAttributes)]
#[deluxe(attributes(delegate))]
struct CommandSetDelegate(Path);

pub fn collect_delegates(_ast: &mut DeriveInput) -> HashMap<Member, &Variant> {
    todo!()
}
