#[derive(Debug)]
pub struct CustomSet<T> {
    data: Vec<T>,
}

impl<T: PartialEq> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.data.len() != other.data.len() {
            return false;
        }
        for x in &self.data {
            if !other.data.contains(x) {
                return false;
            }
        }
        true
    }
}
impl<T: Eq> Eq for CustomSet<T> {}
impl<T: Clone + PartialEq> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut v = Vec::new();
        for x in input {
            if !v.contains(x) {
                v.push(x.clone());
            }
        }
        CustomSet { data: v }
    }
    pub fn contains(&self, element: &T) -> bool {
        for x in &self.data {
            if x == element {
                return true;
            }
        }
        false
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.data.push(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        for x in &self.data {
            if !other.contains(x) {
                return false;
            }
        }
        true
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        for x in &self.data {
            if other.contains(x) {
                return false;
            }
        }
        true
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let mut v = Vec::new();
        for x in &self.data {
            if other.contains(x) && !v.contains(x) {
                v.push(x.clone());
            }
        }
        CustomSet { data: v }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let mut v = Vec::new();
        for x in &self.data {
            if !other.contains(x) && !v.contains(x) {
                v.push(x.clone());
            }
        }
        CustomSet { data: v }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut v = Vec::new();
        for x in &self.data {
            if !v.contains(x) {
                v.push(x.clone());
            }
        }
        for x in &other.data {
            if !v.contains(x) {
                v.push(x.clone());
            }
        }
        CustomSet { data: v }
    }
}