use core::fmt;
macro_rules! array {
    [@INTERNAL $callback:expr; $count:tt] => { ... };
    [| $($rest:tt)*] => { ... };
    [move $($rest:tt)*] => { ... };
    [$expr:expr; $count:tt] => { ... };
}
#[derive(Debug, Clone, Copy)]
pub struct Vector<T, const SIZE: usize> {
    data: [T; SIZE],
}
impl<T: fmt::Display, const SIZE: usize> fmt::Display for Vector<T, SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    where
        T: fmt::Display,
    {
        for d in &self.data {
            write!(f, "{}", d)?
        }
        Ok(())
    }
}
impl<T, const SIZE: usize> Vector<T, SIZE> {
    pub fn move_construct(new_data: Vector<T, SIZE>) -> Self
    where
        T: Copy,
    {
        Self {
            data: new_data.data,
        }
    }
    pub fn create_from_array(new_data: [T; SIZE]) -> Self
    where
        T: Copy,
    {
        Self { data: new_data }
    }
    pub fn default(def_value: T) -> Self
    where
        T: Copy,
    {
        Self {
            data: [def_value; SIZE],
        }
    }
    pub fn get(&self, position: usize) -> &T {
        &self.data[position]
    }
    pub fn set(&mut self, value: T, position: usize) {
        self.data[position] = value;
    }
}

// and we'll implement IntoIterator
impl<T, const SIZE: usize> IntoIterator for Vector<T, SIZE> {
    type Item = T;
    type IntoIter = std::array::IntoIter<Self::Item, SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

pub type Vector2U = Vector<usize, 2>;
