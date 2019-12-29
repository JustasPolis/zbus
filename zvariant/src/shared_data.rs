use std::fmt;
use std::rc::Rc;

// More like shared slice since you can only increment position & decrement len.
// This structure and its data is immutable.
#[derive(Debug, Clone)]
pub struct SharedData {
    data: Rc<Vec<u8>>,
    position: usize,
    end: usize,
}

impl fmt::Display for SharedData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        let bytes = self.bytes();
        let mut first = true;

        for byte in bytes {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{}", byte)?;
        }
        write!(f, "}}")
    }
}

#[allow(clippy::len_without_is_empty)]
impl SharedData {
    pub fn new(data: Vec<u8>) -> Self {
        Self::from(data)
    }

    pub fn subset(&self, index: usize, end: usize) -> Self {
        assert!(end > index);
        assert!(end != 0);

        let mut clone = self.clone();
        clone.position += index;
        clone.end = end + self.position;
        assert!(self.end >= clone.end);

        clone
    }

    pub fn head(&self, len: usize) -> Self {
        self.subset(0, len)
    }

    pub fn tail(&self, index: usize) -> Self {
        self.subset(index, self.len())
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn len(&self) -> usize {
        self.end - self.position
    }

    pub fn bytes(&self) -> &[u8] {
        &self.data[self.position..self.end]
    }
}

impl From<Vec<u8>> for SharedData {
    fn from(value: Vec<u8>) -> Self {
        let end = value.len();

        Self {
            data: Rc::new(value),
            position: 0,
            end,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::SharedData;

    #[test]
    fn shared_data() {
        let data = SharedData::from((0u8..10).collect::<Vec<_>>());
        assert!(data.position() == 0);
        assert!(data.end() == 10);
        assert!(data.len() == 10);

        let subset1 = data.subset(2, 8);
        assert!(subset1.position() == 2);
        assert!(subset1.end() == 8);
        assert!(subset1.len() == 6);

        let subset2 = subset1.subset(3, 5);
        assert!(subset2.position() == 5);
        assert!(subset2.end() == 7);
        assert!(subset2.len() == 2);

        let subset3 = data.head(7);
        assert!(subset3.position() == 0);
        assert!(subset3.end() == 7);
        assert!(subset3.len() == 7);

        let subset4 = subset3.tail(3);
        assert!(subset4.position() == 3);
        assert!(subset4.end() == 7);
        assert!(subset4.len() == 4);

        let subset5 = subset4.subset(0, 4);
        assert!(subset5.position() == 3);
        assert!(subset5.end() == 7);
        assert!(subset5.len() == 4);
    }
}