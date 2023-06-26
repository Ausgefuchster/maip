pub trait Merge {
    /// Merges the other value into self
    /// # Example
    /// ```
    /// use maip::policy::Merge;
    ///
    /// let mut a: Vec<String> = vec!["a".to_string(), "b".to_string()];
    /// let b = vec!["b".to_string(), "c".to_string()];
    /// a.merge(b);
    /// assert_eq!(a, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    /// ```
    fn merge(&mut self, other: Self);
}

impl Merge for Vec<String> {
    fn merge(&mut self, other: Self) {
        other.iter().for_each(|x| {
            if !self.contains(x) {
                self.push(x.clone());
            }
        });
    }
}