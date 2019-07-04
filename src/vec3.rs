use std::ops::*;

use rand::Rng;

pub type Float = f64;
pub const MAX_FLOAT: Float = std::f64::MAX;

#[derive(Clone, Copy, Default, Debug)]
pub struct Vec3(pub Float, pub Float, pub Float);

impl Vec3 {
    #[inline]
    pub fn x(&self) -> Float {
        self.0
    }
    #[inline]
    pub fn y(&self) -> Float {
        self.1
    }
    #[inline]
    pub fn z(&self) -> Float {
        self.2
    }
    #[inline]
    pub fn r(&self) -> Float {
        self.0
    }
    #[inline]
    pub fn g(&self) -> Float {
        self.1
    }
    #[inline]
    pub fn b(&self) -> Float {
        self.2
    }

    #[inline]
    pub fn length(&self) -> Float {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    #[inline]
    pub fn squared_length(&self) -> Float {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    #[inline]
    pub fn make_unit_vector(&mut self) {
        *self /= self.length();
    }

    #[inline]
    pub fn dot(&self, v2: Vec3) -> Float {
        self.0 * v2.0 + self.1 * v2.1 + self.2 * v2.2
    }

    #[inline]
    pub fn cross(&self, v2: Vec3) -> Vec3 {
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
    pub fn random_in_unit_sphere(rng: &mut impl Rng) -> Vec3 {
        loop {
            let p = 2.0 * Vec3(rng.gen::<Float>(), rng.gen::<Float>(), rng.gen::<Float>())
                - Vec3(1., 1., 1.);
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

impl Mul<Vec3> for Float {
    type Output = Vec3;

    #[inline]
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3(self * v.0, self * v.1, self * v.2)
    }
}

impl Div<Float> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, t: Float) -> Vec3 {
        Vec3(self.0 / t, self.1 / t, self.2 / t)
    }
}

impl Mul<Float> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, t: Float) -> Vec3 {
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

impl MulAssign<Float> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, t: Float) {
        self.0 *= t;
        self.1 *= t;
        self.2 *= t;
    }
}

impl DivAssign<Float> for Vec3 {
    #[inline]
    fn div_assign(&mut self, t: Float) {
        let k = 1.0 / t;
        self.0 *= k;
        self.1 *= k;
        self.2 *= k;
    }
}
