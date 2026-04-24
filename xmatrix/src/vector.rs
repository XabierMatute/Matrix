pub struct Vector<K> {
    data: Vec<K>,
}

// General methods for Vector

impl<K> Vector<K> {
    /// Creates a new, empty Vector
    pub fn new() -> Self {
        Vector { data: Vec::new() }
    }

    /// Creates a Vector from a given Vec
    pub fn from_vec(data: Vec<K>) -> Self {
        Vector { data }
    }
}

// Recommended utility functions:

impl<K> Vector<K> {
    /// Returns the length of the vector
    pub fn len(&self) -> usize {
        self.data.len()
    }


}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_len() {
        let vec = Vector { data: vec![1, 2, 3, 4, 5] };
        assert_eq!(vec.len(), 5);
    }
}