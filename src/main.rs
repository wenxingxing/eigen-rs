use rand;

#[derive(Debug)]
pub struct Matrix {
    data: [[u8; 3]; 3],
}

impl Matrix {
    pub fn zeros() -> Matrix {
        Self::new([[0, 0, 0], [0, 0, 0], [0, 0, 0]])
    }
    pub fn new(data: [[u8; 3]; 3]) -> Matrix {
        Matrix { data }
    }

    pub fn det(&self) -> f64 {
        0.0
    }

    pub fn inv(&self) -> Matrix {
        Self::zeros()
    }

    pub fn eigen_values(&self) -> Vec<f64> {
        vec![]
    }

    pub fn eigen_vectors(&self) -> Vec<Matrix> {
        vec![]
    }

    pub fn mul(&self, rhs: &Matrix) -> Matrix {
        Self::zeros()
    }

    pub fn trace(&self) -> f64 {
        0.0
    }

    pub fn diag(&self) -> Matrix {
        Self::zeros()
    }
}
fn main() {
    let x: u64 = rand::random();
    println!("{}", x);

    let m1 = Matrix::new([[1, 2, 3], [2, 3, 4], [3, 4, 5]]);
    m1.det();
    m1.inv();
    m1.eigen_values();
    m1.eigen_vectors();
    m1.diag();
    m1.trace();

    let m2 = Matrix::zeros();

    let m3 = m1.mul(&m2);
}
