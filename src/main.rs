const EPSILON: f64 = 0.000001;

#[derive(Copy, Clone)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }

    fn get_i() -> Self {
        Vector::new(1f64, 0f64, 0f64)
    }

    fn get_j() -> Self {
        Vector::new(0f64, 1f64, 0f64)
    }

    fn get_k() -> Self {
        Vector::new(0f64, 0f64, 1f64)
    }

    fn get_magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn add(&self, other: Self) -> Self {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    fn multiply_by_scalar(&self, scalar: f64) -> Self {
        Vector::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(&self, other: Self) -> Self {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    fn is_parallel_to(&self, other: Self) -> bool {
        if self.get_magnitude() < EPSILON {
            return false;
        }
        if other.get_magnitude() < EPSILON {
            return false;
        }
        self.cross(other).get_magnitude().abs() < EPSILON
    }

    fn is_perpendicular_to(&self, other: Self) -> bool {
        if self.x * self.y * self.z == 0f64 {
            return false;
        }
        if other.x * other.y * other.z == 0f64 {
            return false;
        }
        self.dot(other).abs() < EPSILON
    }

    fn normalize(&self) -> Self {
        self.multiply_by_scalar(1f64 / self.get_magnitude())
    }

    fn is_normalized(&self) -> bool {
        (1f64 - self.get_magnitude()).abs() < EPSILON
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector({}, {}, {})", self.x, self.y, self.z)
    }
}

fn main() {
    let v = Vector::new(3f64, 4f64, 5f64);
    println!("{}", v);
}
