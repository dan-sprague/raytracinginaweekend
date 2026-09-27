use std::ops::{AddAssign,DivAssign,MulAssign,Neg,Index,IndexMut};
use std::ops::{Add,Sub,Mul,Div};
use std::fmt;

#[derive(Default,Clone,Copy)]
pub struct Vec3(pub f64,pub f64,pub f64);
pub type Point3 = Vec3;
pub type Color = Vec3;
#[allow(non_upper_case_globals)]
pub const Point3: fn(f64, f64, f64) -> Vec3 = Vec3;
#[allow(non_upper_case_globals)]
pub const Color: fn(f64, f64, f64) -> Vec3 = Vec3;



impl Vec3 {
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&mut self) -> () {
        let l = self.length();
        *self /= l
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.0,self.1,self.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {

    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
    
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            0: -self.0,
            1: -self.1,
            2: -self.2,
        }
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            0: -self.0,
            1: -self.1,
            2: -self.2
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Attempted to access an illegal index.")
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
         match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Attempted to access an illegal index.")
         }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs:Vec3) -> Vec3 {
        Vec3 { 0: self.0 + rhs.0, 1: self.1 + rhs.1, 2: self.2 + rhs.2}
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self,rhs:Vec3) -> Vec3 {
        Vec3 { 0: self.0 - rhs.0, 1: self.1 - rhs.1, 2: self.2 - rhs.2}
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self,rhs:Vec3) -> Vec3 {
        Vec3 { 0: self.0 * rhs.0, 1: self.1 * rhs.1, 2: self.2 * rhs.2}
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self,t:f64) -> Vec3 {
        Vec3 {0: self.0 * t, 1: self.1 * t, 2: self.2 * t}
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self,rhs:Vec3) -> Vec3 {
        Vec3 { 0: self * rhs.0, 1: self * rhs.1, 2: self * rhs.2}
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self,rhs:f64) -> Vec3 {
       (1.0 / rhs) * self
    }
}


fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.0 * v.0 + u.1 * v.1 + u.2 * v.2
}

fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {0: u.1 * v.2 - u.2 * v.1,
    1: u.2 * v.0 - u.0 * v.2,
    2: u.0 * v.1 - u.1 * v.0,
    }
}

fn unit_vector(u: Vec3) -> Vec3 {
    u / u.length()
}

