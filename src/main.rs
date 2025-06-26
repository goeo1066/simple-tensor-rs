fn main() {
    println!("Hello, world!");
    let mut tensor = Tensor::new(&[2, 2, 3]);
    tensor.set(&[0, 1, 1], 1f32);
    tensor.set(&[1, 1, 2], 3f32);

    println!("(0, 1, 1) = {}", tensor.flatten_pos(&[0, 1, 1]));
    println!("(1, 1, 2) = {}", tensor.flatten_pos(&[1, 1, 2]));

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

    let column = tensor2d.get_column(2);
    println!("{:?}", column);

    let mut tensor3d = Tensor::new(&[2, 2, 2, 2]);
    tensor3d.set(&[0, 0, 0, 0], 1f32);
    tensor3d.set(&[0, 0, 0, 1], 3f32);
    tensor3d.set(&[0, 0, 1, 0], 2f32);
    tensor3d.set(&[0, 0, 1, 1], 4f32);
    tensor3d.set(&[0, 1, 0, 0], 5f32);
    tensor3d.set(&[0, 1, 0, 1], 6f32);
    tensor3d.set(&[0, 1, 1, 0], 7f32);
    tensor3d.set(&[0, 1, 1, 1], 8f32);
    tensor3d.set(&[1, 0, 0, 0], 9f32);
    tensor3d.set(&[1, 0, 0, 1], 10f32);
    tensor3d.set(&[1, 0, 1, 0], 11f32);
    tensor3d.set(&[1, 0, 1, 1], 12f32);
    tensor3d.set(&[1, 1, 0, 0], 13f32);
    tensor3d.set(&[1, 1, 0, 1], 14f32);
    tensor3d.set(&[1, 1, 1, 0], 15f32);
    tensor3d.set(&[1, 1, 1, 1], 16f32);

    for a in 0..2 {
        for b in 0..2 {
            for c in 0..2 {
                for d in 0..2 {
                    println!("{}", tensor3d.get(&[a, b, c, d]));
                }
            }
        }
    }
}

struct Tensor2D {
    row: usize,
    column: usize,
    core: Tensor,
}

impl Tensor2D {
    pub fn new(row: usize, column: usize) -> Self {
        Tensor2D {
            row,
            column,
            core: Tensor::new(&[row, column]),
        }
    }

    pub fn get(&self, row: usize, column: usize) -> f32 {
        self.core.get(&[row, column])
    }

    pub fn set(&mut self, row: usize, column: usize, value: f32) {
        self.core.set(&[row, column], value);
    }

    pub fn get_row(&self, row: usize) -> Box<[f32]> {
        let mut row_buffer = vec![0f32; self.column as usize];
        for c in 0..self.column {
            row_buffer[c as usize] = self.get(row, c);
        }
        row_buffer.into_boxed_slice()
    }

    pub fn get_column(&self, column: usize) -> Box<[f32]> {
        let mut column_buffer = vec![0f32; self.row as usize];
        for r in 0..self.row {
            column_buffer[r as usize] = self.get(r, column);
        }
        column_buffer.into_boxed_slice()
    }
}

struct Tensor {
    shape: Box<[usize]>,
    array: Box<[f32]>,
}

impl Tensor {
    pub fn new(shape: &[usize]) -> Self {
        let size = {
            let mut size = 1;
            for x in shape {
                size *= x;
            }
            size
        };

        let array: Box<[f32]> = if size == 0 {
            vec![].into_boxed_slice()
        } else {
            vec![0f32; size].into_boxed_slice()
        };

        Tensor {
            shape: shape.into(),
            array,
        }
    }

    pub fn set(&mut self, pos: &[usize], value: f32) {
        self.assert_match_shape(pos);
        let idx = self.flatten_pos(pos);
        self.array[idx] = value;
    }

    pub fn get(&self, pos: &[usize]) -> f32 {
        self.assert_match_shape(pos);
        let idx = self.flatten_pos(pos);
        self.array[idx]
    }

    fn flatten_pos(&self, pos: &[usize]) -> usize {
        let mut result = pos[pos.len() - 1];
        let mut pos_base = 1;

        for i in (0..pos.len() - 1).rev() {
            pos_base *= self.shape[i];
            result += pos[i] * pos_base;
        }

        result
    }

    fn assert_match_shape(&self, shape: &[usize]) {
        if self.shape.len() != shape.len() {
            panic!(
                "Shape mismatch: expected {}, got {}",
                self.shape.len(),
                shape.len()
            );
        }
    }
}
