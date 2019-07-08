use std::ops::*;

use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    #[inline]
    pub fn x(&self) -> f64 {
        self.0
    }
    #[inline]
    pub fn y(&self) -> f64 {
        self.1
    }
    #[inline]
    pub fn z(&self) -> f64 {
        self.2
    }
    #[inline]
    pub fn r(&self) -> f64 {
        self.0
    }
    #[inline]
    pub fn g(&self) -> f64 {
        self.1
    }
    #[inline]
    pub fn b(&self) -> f64 {
        self.2
    }

    #[inline]
    pub fn length(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    #[inline]
    pub fn squared_length(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    #[inline]
    pub fn make_unit_vector(&mut self) {
        *self /= self.length();
    }

    #[inline]
    pub fn dot(&self, v2: &Vec3) -> f64 {
        self.0 * v2.0 + self.1 * v2.1 + self.2 * v2.2
    }

    #[inline]
    pub fn cross(&self, v2: &Vec3) -> Vec3 {
        Vec3(
            self.1 * v2.2 - self.2 * v2.1,
            -(self.0 * v2.2 - self.2 * v2.0),
            self.0 * v2.1 - self.1 * v2.0,
        )
    }

    #[inline]
    pub fn into_unit(self) -> Vec3 {
        self / self.length()
    }

    #[inline]
    pub fn reflect(self, n: &Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * *n
    }

    #[inline]
    pub fn random_in_unit_sphere(rng: &mut impl Rng) -> Vec3 {
        loop {
            let p =
                2.0 * Vec3(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) - Vec3(1., 1., 1.);
            if p.squared_length() < 1.0 {
                return p;
            }
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, other: Vec3) -> Vec3 {
        Vec3(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3(self * v.0, self * v.1, self * v.2)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, t: f64) -> Vec3 {
        Vec3(self.0 / t, self.1 / t, self.2 / t)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, t: f64) -> Vec3 {
        Vec3(self.0 * t, self.1 * t, self.2 * t)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Vec3) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: Vec3) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

impl MulAssign for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: Vec3) {
        self.0 *= other.0;
        self.1 *= other.1;
        self.2 *= other.2;
    }
}

impl DivAssign for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: Vec3) {
        self.0 /= other.0;
        self.1 /= other.1;
        self.2 /= other.2;
    }
}

impl MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, t: f64) {
        self.0 *= t;
        self.1 *= t;
        self.2 *= t;
    }
}

impl DivAssign<f64> for Vec3 {
    #[inline]
    fn div_assign(&mut self, t: f64) {
        let k = 1.0 / t;
        self.0 *= k;
        self.1 *= k;
        self.2 *= k;
    }
}
