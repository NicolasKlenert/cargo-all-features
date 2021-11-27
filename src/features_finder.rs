use itertools::Itertools;

pub fn fetch_feature_sets(package: &crate::cargo_metadata::Package) -> Vec<Vec<String>> {
    let mut features = vec![];
    let filter_denylist = |f: &String| !package.denylist.contains(f);

    if package.allowlist.is_empty() {
        if !package.skip_optional_dependencies {
            features.extend(fetch_optional_dependencies(package).filter(filter_denylist));
        }
        features.extend(fetch_features(package).filter(filter_denylist));
        features.extend(
            package
                .extra_features
                .iter()
                .cloned()
                .filter(filter_denylist),
        );
    } else {
        // allowlist cannot be mixed with denylist or any of the other above options,
        // no need to filter
        features.extend(package.allowlist.iter().cloned())
    };

    let mut feature_sets = vec![];

    for n in 0..=features.len() {
        'outer: for feature_set in features.iter().combinations(n) {
            'inner: for skip_feature_set in &package.skip_feature_sets {
                for feature in skip_feature_set {
                    if !feature_set.contains(&feature) {
                        // skip_feature_set does not match
                        continue 'inner;
                    }
                }
                // skip_feature_set matches: do not add it to feature_sets
                continue 'outer;
            }
            feature_sets.push(feature_set.iter().map(|n| n.to_string()).collect());
        }
    }

    feature_sets
}

fn fetch_optional_dependencies(
    package: &crate::cargo_metadata::Package,
) -> impl Iterator<Item = String> + '_ {
    package
        .dependencies
        .iter()
        .filter(|dependency| dependency.optional)
        .map(|dependency| {
            if let Some(name) = &dependency.rename {
                name.to_string()
            } else {
                dependency.name.to_string()
            }
        })
}

fn fetch_features(package: &crate::cargo_metadata::Package) -> impl Iterator<Item = String> + '_ {
    package
        .features
        .iter()
        .filter(|key| key != &"default")
        // Some crates use "__" to indicate internal features
        .filter(|key| !key.starts_with("__"))
        .cloned()
}
