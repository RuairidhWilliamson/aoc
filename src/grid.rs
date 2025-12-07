use std::borrow::Cow;

pub struct ByteGrid<'a> {
    contents: Cow<'a, [u8]>,
    width: usize,
    height: usize,
}

impl<'a> ByteGrid<'a> {
    pub fn new(contents: impl Into<Cow<'a, [u8]>>) -> Self {
        let contents = contents.into();
        assert!(contents.is_ascii());
        let width = contents
            .as_ref()
            .iter()
            .enumerate()
            .find(|(_, x)| **x == b'\n')
            .unwrap()
            .0;
        let height = contents.len() / (width + 1);
        assert_eq!((width + 1) * height, contents.len());
        Self {
            contents,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(unsafe { self.get_unchecked(x, y) })
    }

    pub unsafe fn get_unchecked(&self, x: usize, y: usize) -> u8 {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        let index = x + y * (self.width + 1);
        debug_assert!(index < self.contents.len());
        unsafe { *self.contents.get_unchecked(index) }
    }

    pub fn set(&mut self, x: usize, y: usize, c: u8) -> Result<(), ()> {
        if x < self.width && y < self.height {
            unsafe {
                self.set_unchecked(x, y, c);
            }
            Ok(())
        } else {
            Err(())
        }
    }

    pub unsafe fn set_unchecked(&mut self, x: usize, y: usize, c: u8) {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        let index = x + y * (self.width + 1);
        debug_assert!(index < self.contents.len());
        unsafe {
            *self.contents.to_mut().get_unchecked_mut(index) = c;
        }
    }

    pub fn adjacent(&self, x: usize, y: usize) -> impl Iterator<Item = u8> {
        [
            (x.checked_sub(1), y.checked_sub(1)),
            (x.checked_add(0), y.checked_sub(1)),
            (x.checked_add(1), y.checked_sub(1)),
            (x.checked_sub(1), y.checked_add(0)),
            (x.checked_add(1), y.checked_add(0)),
            (x.checked_sub(1), y.checked_add(1)),
            (x.checked_add(0), y.checked_add(1)),
            (x.checked_add(1), y.checked_add(1)),
        ]
        .into_iter()
        .filter_map(|(x, y)| self.get(x?, y?))
    }

    pub fn display_ascii(&self) -> Option<&str> {
        str::from_utf8(&self.contents).ok().filter(|s| s.is_ascii())
    }
}
