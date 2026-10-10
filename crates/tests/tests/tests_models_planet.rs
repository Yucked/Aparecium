#[cfg(test)]
mod tests {
    use aparecium::{SchemaGetter, SetterArray};
    use aparecium_tests::*;

    #[test]
    fn test_auto_inference() {
        Planet::<u64>::schema().tags.append(45u64);
        PlanetX::<u64>::schema();
        PlanetX::<u64>::schema().tags.append(45u64);
        // PlanetX::<u64>::schema().tags.append("rer");
        // PlanetX::<u64>::schema().data.equal_to(45u64);
    }
}
