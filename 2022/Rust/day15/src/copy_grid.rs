#[derive(Debug, PartialEq, Eq)]
pub struct CopyGrid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Copy> CopyGrid<T> {
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> T {
        self.data[y * self.width + x]
    }

    fn entries(&self) -> impl Iterator<Item = ((usize, usize), T)> + '_ {
        (0..self.height())
            .flat_map(|y| (0..self.width()).map(move |x| (x, y)))
            .map(move |(x, y)| ((x, y), self.data[y * self.width() + x]))
    }

    #[inline]
    fn new_copied(width: usize, height: usize, t: T) -> Self {
        Self {
            data: (0..width * height).map(|_| t).collect(),
            width,
            height,
        }
    }
}

#[allow(unused)]
impl<T> CopyGrid<T> {
    #[inline]
    fn new(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        Self {
            data: (0..width * height).map(|_| Default::default()).collect(),
            width,
            height,
        }
    }

    #[inline]
    pub fn with_data(data: Vec<T>, width: usize, height: usize) -> Self {
        Self {
            data,
            width,
            height,
        }
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, v: T) -> T {
        let w = self.width();
        std::mem::replace(&mut self.data[y * w + x], v)
    }

    #[inline]
    pub fn get_ref(&self, x: usize, y: usize) -> &T {
        &self.data[y * self.width + x]
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }
}
