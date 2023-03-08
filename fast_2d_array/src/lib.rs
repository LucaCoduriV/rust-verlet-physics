#[derive(Debug)]
pub struct Array2D<T: Default> {
    height: usize,
    width: usize,
    data: Vec<T>,
}

impl<T: Default> Array2D<T>
    where T: Default + Clone {
    pub fn new(height: usize, width: usize) -> Self {
        let data = vec![T::default(); height * width];

        Self {
            data,
            height,
            width,
        }
    }

    pub fn insert(&mut self, data:T, x:usize, y:usize){
        self.data[x + y * self.width] = data;
    }

    pub fn try_get_mut_as_1d(&mut self, i:usize) -> Option<&mut T>{
        if i > self.height * self.width {
            return None;
        }
        Some(&mut self.data[i])
    }

    pub fn get_mut_as_1d(&mut self, i:usize) -> &mut T{
        &mut self.data[i]
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.data[x + y * self.width]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.data[x + y * self.width]
    }

    pub fn try_get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if self.is_in_bounds(x,y) {
            return None;
        }
        Some(&mut self.data[x + y * self.width])
    }

    pub fn try_get(&self, x: usize, y: usize) -> Option<&T> {
        if self.is_in_bounds(x,y) {
            return None;
        }
        Some(&self.data[x + y * self.width])
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn total_size(&self) -> usize {
        self.width * self.height
    }

    fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width || y < self.height
    }
}


#[cfg(test)]
mod test {
    use crate::Array2D;

    #[test]
    fn test() {
        let mut array: Array2D<String> = Array2D::new(10, 10);
        array.insert(String::from("coucou"), 0, 9);

        println!("{}", array.get(0,9));
        println!("{:?}", array.try_get(0,8));

    }
}
