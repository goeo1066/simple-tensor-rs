fn main() {
    println!("Hello, world!");
    let mut tensor = Tensor::new(&[2, 2, 3]);
    tensor.set(&[0, 1, 1], 1f32);
    tensor.set(&[1, 1, 2], 3f32);

    println!("(0, 1, 1) = {}", tensor.get(&[0, 1, 1]));
    println!("(1, 1, 2) = {}", tensor.get(&[1, 1, 2]));

    let mut tensor2d = Tensor2D::new(3, 3);
    tensor2d.set(0, 0, 1f32);
    tensor2d.set(0, 1, 2f32);
    tensor2d.set(0, 2, 3f32);

    tensor2d.set(1, 0, 4f32);
    tensor2d.set(1, 1, 5f32);
    tensor2d.set(1, 2, 6f32);

    tensor2d.set(2, 0, 7f32);
    tensor2d.set(2, 1, 8f32);
    tensor2d.set(2, 2, 9f32);


    println!("{:?}", tensor2d.get_row(0));
    println!("{:?}", tensor2d.get_row(1));
    println!("{:?}", tensor2d.get_row(2));



    let column= tensor2d.get_column(2);
    println!("{:?}", column);
}

struct Tensor2D {
    row: u32,
    column: u32,
    core: Tensor,
}

impl Tensor2D {
    pub fn new(row: u32, column: u32) -> Self {
        Tensor2D {
            row,
            column,
            core: Tensor::new(&[row, column]),
        }
    }

    pub fn get(&self, row: u32, column: u32) -> f32 {
        self.core.get(&[row, column])
    }

    pub fn set(&mut self, row: u32, column: u32, value: f32) {
        self.core.set(&[row, column], value);
    }

    pub fn get_row(&self, row: u32) -> Box<[f32]> {
        let mut row_buffer = vec![0f32; self.column as usize];
        for c in 0..self.column {
            row_buffer[c as usize] = self.get(row, c);
        }
        row_buffer.into_boxed_slice()
    }

    pub fn get_column(&self, column: u32) -> Box<[f32]> {
        let mut column_buffer = vec![0f32; self.row as usize];
        for r in 0..self.row {
            column_buffer[r as usize] = self.get(r, column);
        }
        column_buffer.into_boxed_slice()
    }
}

struct Tensor {
    shape: Box<[u32]>,
    array: Box<[f32]>,
}

impl Tensor {
    pub fn new(shape: &[u32]) -> Self {
        let array: Box<[f32]>;
        let mut size = 1u32;

        for x in shape {
            size *= x;
        }

        if size == 0 {
            array = vec![].into_boxed_slice();
        } else {
            array = vec![0f32; size as usize].into_boxed_slice();
        }

        Tensor {
            shape: shape.into(),
            array,
        }
    }

    pub fn set(&mut self, pos: &[u32], value: f32) {
        self.assert_match_shape(pos);
        let idx = self.flatten_pos(pos);
        self.array[idx] = value;
    }

    pub fn get(&self, pos: &[u32]) -> f32 {
        self.assert_match_shape(pos);
        let idx = self.flatten_pos(pos);
        self.array[idx]
    }

    fn flatten_pos(&self, pos: &[u32]) -> usize {
        let mut base = 0;

        for i in 0..pos.len() - 1 {
            let p = pos[i];
            base += p * self.shape[i];
        }

        base += pos[pos.len() - 1];
        base as usize
    }

    fn assert_match_shape(&self, shape: &[u32]) {
        if self.shape.len() != shape.len() {
            panic!(
                "Shape mismatch: expected {}, got {}",
                self.shape.len(),
                shape.len()
            );
        }
    }
}
