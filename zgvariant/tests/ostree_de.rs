#![cfg(feature = "ostree-tests")]

#[test]
fn ostree_de() {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use zgvariant::{LE, Type, Value, serialized::Context};

    #[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
    struct Summary<'a>(Vec<Repo<'a>>, #[serde(borrow)] HashMap<&'a str, Value<'a>>);

    #[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
    struct Repo<'a>(&'a str, #[serde(borrow)] Metadata<'a>);

    #[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
    struct Metadata<'a>(u64, Vec<u8>, #[serde(borrow)] HashMap<&'a str, Value<'a>>);

    let encoded = std::fs::read("../test-data/flatpak-summary.dump").unwrap();
    let ctxt = Context::new(LE, 0);
    let encoded = zgvariant::serialized::Data::new(encoded, ctxt);
    let _: Summary<'_> = encoded.deserialize().unwrap().0;
    // If we're able to deserialize all the data successfully, don't bother checking the summary
    // data.
}
