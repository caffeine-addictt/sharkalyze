use std::collections::HashSet;

/// Calcuates the overlap between two strings
pub fn calculate_overlap(s1: &str, s2: &str) -> f32 {
    if s1.is_empty() || s2.is_empty() {
        return (s1.len() == s2.len()) as usize as f32;
    }

    let s1_set: HashSet<char> = HashSet::from_iter(s1.chars());
    let s2_set: HashSet<char> = HashSet::from_iter(s2.chars());
    let common = s1_set.intersection(&s2_set).count();

    (common * 2) as f32 / (s1.len() + s2.len()) as f32
}
