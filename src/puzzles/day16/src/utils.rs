use itertools::Itertools;

pub fn combinations_taking_one_from_each<T: Clone>(sets: Vec<Vec<T>>) -> Vec<Vec<T>> {
    sets.into_iter().fold(
        vec![vec![]],
        |combinations_of_all_sets_so_far, variants_of_one_set| {
            combinations_of_all_sets_so_far
                .into_iter()
                .cartesian_product(variants_of_one_set)
                .into_iter()
                .map(|(mut combination, variant)| {
                    combination.push(variant);
                    combination
                })
                .collect()
        },
    )
}
