pub struct MatrixStruct<T: Copy> {
    pub matrix: Vec<T>,
    pub num_rows: usize,
    pub num_columns: usize,
}

impl<T: Copy + std::fmt::Display> MatrixStruct<T> {
    pub fn at(&self, place: (usize, usize)) -> &T {
        assert_eq!(self.num_rows > place.0, true);
        assert_eq!(self.num_columns > place.1, true);
        &self.matrix[(place.0 * self.num_columns) + place.1]
    }

    pub fn print(&self) {
        for i in 0..self.num_rows {
            let start = i * self.num_columns;
            let end = start + self.num_columns;
            for j in start..end {
                print!("{} ", self.matrix[j]);
            }
            println!("\n");
        }
    }
}
