pub trait Merge {
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