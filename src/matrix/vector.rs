extern crate num_traits;
use num_traits::Float;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub type Vector3D = Vector<f32, 3>;
// pub type Vector2D = Vector<f32, 2>;
pub type Point3D = Vector3D;
// pub type Point2D = Vector2D;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector<T: Float, const DIM: usize> {
    data: [T; DIM],
}

impl Vector<f32, 3> {
    #[inline]
    pub fn x(&self) -> f32 {
        return self.data[0];
    }

    #[inline]
    pub fn y(&self) -> f32 {
        return self.data[1];
    }

    #[inline]
    pub fn z(&self) -> f32 {
        return self.data[2];
    }
}

#[allow(dead_code)]
impl<T: Float, const DIM: usize> Vector<T, DIM> {
    pub fn new(data: [T; DIM]) -> Vector<T, DIM> {
        //        return Vector::<T, DIM> { data: data };
        return Self { data: data };
    }

    pub fn zero() -> Vector<T, DIM> {
        return Vector::<T, DIM>::new([T::zero(); DIM]);
    }

    pub fn one() -> Vector<T, DIM> {
        return Vector::<T, DIM>::new([T::one(); DIM]);
    }

    pub fn norm_squared(&self) -> T {
        let mut result = T::zero();
        for i in 0..DIM {
            result = result + self.data[i] * self.data[i];
        }
        return result;
    }

    pub fn norm(&self) -> T {
        return self.norm_squared().sqrt();
    }

    pub fn normalized(&self) -> Vector<T, DIM> {
        let norm = self.norm();
        let mut result = [T::zero(); DIM];
        for i in 0..DIM {
            result[i] = self.data[i] / norm;
        }
        return Self::new(result);
    }

    pub fn dot(&self, other: &Self) -> T {
        let mut sum = T::zero();
        for i in 0..DIM {
            sum = sum + self.data[i] * other.data[i];
        }
        return sum;
    }

    pub fn cross(&self, other: &Self) -> Vector<T, DIM> {
        if DIM < 2 {
            return Self::new([T::zero(); DIM]);
        }

        let mut result: [T; DIM] = [T::zero(); DIM];
        result[DIM - 1] = self.data[0] * other.data[1] - self.data[1] * other.data[0];
        result[DIM - 2] = self.data[DIM - 1] * other.data[0] - self.data[0] * other.data[DIM - 1];
        for i in 0..(DIM - 2) {
            result[i] =
                -(self.data[i + 2] * other.data[i + 1] - self.data[i + 1] * other.data[i + 2]);
        }

        return Self::new(result);
    }

    pub fn scale(&self, scale: T) -> Vector<T, DIM> {
        let mut result = [T::zero(); DIM];
        for i in 0..DIM {
            result[i] = self.data[i] * scale;
        }
        return Self::new(result);
    }
}

// Vector Addition
impl<T: Float, const DIM: usize> Add<Self> for Vector<T, DIM> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut result: [T; DIM] = [T::zero(); DIM];
        for i in 0..DIM {
            result[i] = self.data[i] + other.data[i];
        }
        return Self::new(result);
    }
}

// Vector + scalar adds scalar to each element of the vector.
impl<T: Float, const DIM: usize> Add<T> for Vector<T, DIM> {
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        let mut result: [T; DIM] = [T::zero(); DIM];
        for i in 0..DIM {
            result[i] = self.data[i] + other;
        }
        return Self::new(result);
    }
}

// Vector Subtraction
impl<T: Float, const DIM: usize> Sub for Vector<T, DIM> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut result: [T; DIM] = [T::zero(); DIM];
        for i in 0..DIM {
            result[i] = self.data[i] - other.data[i];
        }
        return Self::new(result);
    }
}

// Unary Negation
impl<T: Float, const DIM: usize> Neg for Vector<T, DIM> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut result: [T; DIM] = [T::zero(); DIM];
        for i in 0..DIM {
            result[i] = -self.data[i];
        }
        return Self::new(result);
    }
}

// Dot product
impl<T: Float, const DIM: usize> Mul<Self> for Vector<T, DIM> {
    type Output = T;

    fn mul(self, other: Self) -> T {
        return self.dot(&other);
    }
}

// Scalar Multiplication
impl<T: Float, const DIM: usize> Mul<T> for Vector<T, DIM> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        return self.scale(other);
    }
}

// Scalar Division
impl<T: Float, const DIM: usize> Div<T> for Vector<T, DIM> {
    type Output = Self;

    fn div(self, other: T) -> Self {
        return self.scale(T::one() / other);
    }
}

impl<T: Float + fmt::Display> fmt::Display for Vector<T, 3> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // We need to remove "-" from the number output.
        formatter.write_fmt(format_args!(
            "Vec3D[{}, {}, {}]",
            self.data[0], self.data[1], self.data[2]
        ))
    }
}
impl<T: Float + fmt::Display> fmt::Display for Vector<T, 2> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // We need to remove "-" from the number output.
        formatter.write_fmt(format_args!("Vec2D[{}, {}]", self.data[0], self.data[1]))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn norm() {
        let vec = Vector3D {
            data: [2.0, 3.0, 6.0],
        };
        assert_eq!(vec.norm_squared(), 49.0);
        assert_eq!(vec.norm(), 7.0);
    }

    #[test]
    fn normalize() {
        let vec = Vector::<f32, 4> {
            data: [8.0, 8.0, 8.0, 8.0],
        };
        assert_eq!(
            vec.normalized(),
            Vector::<f32, 4> {
                data: [0.5, 0.5, 0.5, 0.5]
            }
        )
    }

    #[test]
    fn add() {
        let vec1 = Vector3D::new([1.0, 2.0, 3.0]);
        let vec2 = Vector3D::new([4.0, 5.0, 6.0]);
        let result = Vector3D::new([5.0, 7.0, 9.0]);
        assert_eq!(vec1.add(vec2), result);
        assert_eq!(vec1 + vec2, result);
    }

    #[test]
    fn sub() {
        let vec1 = Vector3D::new([1.0, 2.0, 3.0]);
        let vec2 = Vector3D::new([4.0, 5.0, 6.0]);
        let result = Vector3D::new([-3.0, -3.0, -3.0]);

        assert_eq!(vec1.sub(vec2), result);
        assert_eq!(vec1 - vec2, result);
    }

    #[test]
    fn scale() {
        let vec1 = Vector3D {
            data: [5.0, 9.0, 1.0],
        };
        assert_eq!(
            vec1.scale(2.0),
            Vector3D {
                data: [10.0, 18.0, 2.0]
            }
        );
    }

    #[test]
    fn dot() {
        let vec1 = Vector3D {
            data: [1.0, 2.0, 3.0],
        };
        let vec2 = Vector3D {
            data: [3.0, 1.0, 2.0],
        };
        let vec3 = Vector3D {
            data: [4.0, 1.0, -2.0],
        };
        assert_eq!(vec1.dot(&vec2), 11.0);
        assert_eq!(vec1.dot(&vec3), 0.0);
        assert_eq!(vec3.dot(&vec1), 0.0);
    }

    #[test]
    fn cross() {
        let vec1 = Vector3D {
            data: [1.0, 2.0, 3.0],
        };
        let vec2 = Vector3D {
            data: [2.0, 4.0, 6.0],
        };
        let vec3 = Vector3D {
            data: [8.0, 7.0, 2.0],
        };
        let vec4 = Vector3D {
            data: [3.0, 1.0, 5.0],
        };
        assert_eq!(
            vec1.cross(&vec2),
            Vector3D {
                data: [0.0, 0.0, 0.0]
            }
        );
        assert_eq!(
            vec2.cross(&vec1),
            Vector3D {
                data: [0.0, 0.0, 0.0]
            }
        );
        assert_eq!(
            vec3.cross(&vec4),
            Vector3D {
                data: [33.0, -34.0, -13.0]
            }
        );
        assert_eq!(
            vec4.cross(&vec3),
            Vector3D {
                data: [-33.0, 34.0, 13.0]
            }
        );
    }
}
