// Rust port of FTS fuzzy-match algorithm

pub mod algo;

#[cfg(test)]
mod tests {
    use super::algo;

    #[test]
    fn fuzzy_match_simple_correct() {
        assert_eq!(algo::fuzzy_match_simple("Fuz", "Wuzzfuzzy"), true);
    }

    #[test]
    #[should_panic]
    fn fuzzy_match_simple_wrong() {
        assert_eq!(algo::fuzzy_match_simple("Faz", "WuzzFuzzy"), true);
    }

    #[test]
    fn fuzzy_match_hi() {
        assert_eq!(algo::fuzzy_match("lib", "LightingBuildInfo.h"), (true, 10,  vec![0, 1, 8]));
    }

    #[test]
    fn fuzzy_match_low() {
        assert_eq!(algo::fuzzy_match("lib", "EnvQueryContext_BlueprintBase.cpp"), (true, -29, vec![17, 22, 25]));
    }
}
