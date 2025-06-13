use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct SparseMatrix2D {
    pub data: Vec<Option<Vec<u8>>>,
    pub shape: (usize, usize),
}

impl fmt::Display for SparseMatrix2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for vec in &self.data {
            match vec {
                Some(r) => {
                    for val in r {
                        write!(f, "{} ", val)?;
                    }
                }
                None => {
                    for _ in 0..self.shape.1 {
                        write!(f, "0 ")?;
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl SparseMatrix2D {

    #[inline(always)]
    pub fn get(&self, a0: usize, a1: usize) -> Option<u8> {
        self.data.get(a0)
            .and_then(|vector| vector.as_ref())
            .and_then(|vector| vector.get(a1).copied())
    }

    #[inline(always)]
    pub fn get_checked(&self, a0: usize, a1: usize) -> u8 {
        self.data[a0]
            .as_ref()
            .expect("0th axis is None")[a1]
    }

    pub fn xor_inline(&mut self, other: &SparseMatrix2D) {
        for (a0_index, other_vec_opt) in other.data.iter().enumerate() {
            if other_vec_opt.is_none() { continue; }
            let vec = self.data[a0_index].get_or_insert_with(|| vec![0; self.shape.1]);

            for (a1_index, val) in other_vec_opt.as_ref().unwrap().iter().enumerate() {
                vec[a1_index] ^= val;
            }

        }
    }

    pub fn xor(&self, other: &SparseMatrix2D) -> SparseMatrix2D {
        let mut ret: Self = self.clone();
        ret.xor_inline(other);
        ret
    }
    
    pub fn xor_inline_by_idx(&mut self, a0: usize, a1: usize, val: u8) -> &mut SparseMatrix2D {
        self.insert(a0, a1, self.get(a0, a1).unwrap_or(0) ^ val);
        self
    }

    pub fn new(a0_len: usize, a1_len: usize) -> Self {
        SparseMatrix2D {
            data: vec![None; a0_len],
            shape: (a0_len, a1_len),
        }
    }

    pub fn new_by_shape(shape: (usize, usize)) -> Self {
        Self::new(shape.0, shape.1)
    }

    pub fn new_like(other: &Self) -> Self {
        Self::new_by_shape(other.shape)
    }
}

impl SparseMatrix2D {
    pub fn insert(&mut self, a0_idx: usize, a1_idx: usize, value: u8) {
        if a0_idx >= self.shape.0 || a1_idx >= self.shape.1 {
            return;
        }

        if self.data[a0_idx].is_none() {
            self.data[a0_idx] = Some(vec![0; self.shape.1]);
        }

        if let Some(row_data) = self.data[a0_idx].as_mut() {
            row_data[a1_idx] = value;
        }
    }

    pub fn get_nnz(&self) -> Vec<(usize, usize, u8)> {
        let mut result = Vec::new();

        for (a0_index, row_opt) in self.data.iter().enumerate() {
            if let Some(row) = row_opt {
                for (a1_index, &value) in row.iter().enumerate() {
                    if value != 0 {
                        result.push((a0_index, a1_index, value));
                    }
                }
            }
        }

        result
    }
}
