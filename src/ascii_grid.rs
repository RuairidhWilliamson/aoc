use std::borrow::Cow;

#[derive(Debug)]
pub struct OutOfBounds;

pub struct AsciiGrid<'a> {
    contents: Cow<'a, [u8]>,
    width: usize,
    height: usize,
}

impl<'a> AsciiGrid<'a> {
    pub fn new(contents: impl Into<Cow<'a, [u8]>>) -> Self {
        let mut contents = contents.into();
        assert!(contents.is_ascii());
        match &mut contents {
            Cow::Borrowed(slice) => {
                *slice = slice.strip_suffix(b"\n").unwrap_or(slice);
            }
            Cow::Owned(owned) => {
                *owned = owned.strip_suffix(b"\n").unwrap_or(owned).to_vec();
            }
        }
        let width = contents
            .as_ref()
            .iter()
            .enumerate()
            .find(|(_, x)| **x == b'\n')
            .unwrap()
            .0;
        let length = contents.len();
        let height = (length + 1) / (width + 1);
        assert_eq!((width + 1) * height, (length + 1));
        Self {
            contents,
            width,
            height,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = u8> {
        self.contents.iter().copied().filter(|b| *b != b'\n')
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

    /// # Safety
    /// x < width and y < height
    pub unsafe fn get_unchecked(&self, x: usize, y: usize) -> u8 {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        let index = x + y * (self.width + 1);
        debug_assert!(index < self.contents.len());
        unsafe { *self.contents.get_unchecked(index) }
    }

    pub fn set(&mut self, x: usize, y: usize, c: u8) -> Result<(), OutOfBounds> {
        if x < self.width && y < self.height {
            unsafe {
                self.set_unchecked(x, y, c);
            }
            Ok(())
        } else {
            Err(OutOfBounds)
        }
    }

    /// # Safety
    /// x < width and y < height
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
