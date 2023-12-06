use std::ops::Mul;

use super::{point::Point, tuple::Tuple, vector::Vector};

#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub struct Matrice {
    pub size: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrice {
    pub fn new(size: usize) -> Self {
        Matrice {
            size,
            data: vec![vec![0.0; size]; size],
        }
    }

    pub fn inverse(&self) -> Self {
        let det = self.determinant();
        if det == 0.0 {
            panic!("Non invertible matrix")
        }
        let mut out = Self::new(self.size);
        let mut c;
        for row in 0..self.size {
            for col in 0..self.size {
                c = self.cofactor(row, col);
                out.write_element(col, row, c / det);
            }
        }
        out
    }
    pub fn determinant(&self) -> f64 {
        if self.size == 2 {
            return self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0];
        }
        let mut det = 0.0;
        for c in 0..self.size {
            det += self.data[0][c] * self.cofactor(0, c);
        }
        det
    }

    pub fn submatrix(&self, r: usize, c: usize) -> Self {
        let mut data = vec![];
        let mut row_to_add = vec![];
        for row in 0..self.size {
            if row == r {
                continue;
            }
            for column in 0..self.size {
                if column == c {
                    continue;
                }
                row_to_add.push(self.element_at(row, column));
            }
            data.push(row_to_add);
            row_to_add = vec![];
        }
        Self {
            size: self.size - 1,
            data,
        }
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn element_at(&self, row: usize, column: usize) -> f64 {
        self.data[row][column]
    }
    pub fn write_element(&mut self, row: usize, column: usize, element: f64) {
        self.data[row][column] = element;
    }

    pub fn identity() -> Self {
        let mut out = Matrice::new(4);
        for ix in 0..out.size() {
            out.write_element(ix, ix, 1.0);
        }
        out
    }
    pub fn transpose(&self) -> Self {
        let mut out = Self::new(self.size());
        for ix in 0..self.size {
            for jx in 0..self.size {
                out.write_element(ix, jx, self.element_at(jx, ix));
            }
        }
        out
    }

    pub fn minor(&self, row: usize, column: usize) -> f64 {
        self.submatrix(row, column).determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f64 {
        if (row + column) % 2 != 0 {
            return -self.minor(row, column);
        }
        self.minor(row, column)
    }
}

impl Default for Matrice {
    fn default() -> Self {
        Self::identity()
    }
}

impl Mul for Matrice {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = Self::new(self.size);
        let mut element = 0.0;
        for ix in 0..self.size {
            for jx in 0..self.size {
                for kx in 0..self.size {
                    element += self.element_at(ix, kx) * rhs.element_at(kx, jx);
                }
                out.write_element(ix, jx, element);
                element = 0.0;
            }
        }
        out
    }
}

impl Mul<&Vector> for &Matrice {
    type Output = Vector;
    fn mul(self, rhs: &Vector) -> Self::Output {
        let tuple = [rhs.position.x, rhs.position.y, rhs.position.z, 0.0];
        let mut out = [0.0; 4];
        for (ix, row) in self.data.iter().enumerate() {
            for (jx, col) in row.iter().enumerate() {
                out[ix] += *col * tuple[jx];
            }
        }
        Vector::from(Tuple::new(out[0], out[1], out[2]))
    }
}

impl Mul<&Point> for &Matrice {
    type Output = Point;
    fn mul(self, rhs: &Point) -> Self::Output {
        let tuple = [rhs.position.x, rhs.position.y, rhs.position.z, 1.0];
        let mut out = [0.0; 4];
        for (ix, row) in self.data.iter().enumerate() {
            for (jx, col) in row.iter().enumerate() {
                out[ix] += *col * tuple[jx];
            }
        }
        Point::from(Tuple::new(out[0], out[1], out[2]))
    }
}
