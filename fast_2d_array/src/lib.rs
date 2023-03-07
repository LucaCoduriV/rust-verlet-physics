use std::marker::PhantomData;

struct Array2D<'a, T: Default> {
    height: usize,
    width: usize,
    data: Vec<Option<T>>,
    phantom_data: PhantomData<&'a T>,
}

impl<'a, T: Default> Array2D<'a, T>
    where T: Default + Clone {
    pub fn new(height: usize, width: usize) -> Self {
        let data = vec![None; height * width];

        Self {
            data,
            phantom_data: PhantomData::default(),
            height,
            width,
        }
    }

    pub fn insert(&mut self, data:T, x:usize, y:usize){
        self.data[x + y * self.width] = Some(data);
    }

    pub fn get(&'a self, x: usize, y: usize) -> &'a T {
        self.data[x + y * self.width].as_ref().unwrap()
    }

    pub fn try_get(&'a self, x: usize, y: usize) -> Option<&'a T> {
        self.data[x + y * self.width].as_ref()
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
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
