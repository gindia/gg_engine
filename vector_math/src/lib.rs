use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// Utils/Extra {{{

pub trait ExtraF32 {
    fn lerp(from: f32, to: f32, t: f32) -> f32;
    fn step(from: f32, to: f32, step: f32) -> f32;
}

impl ExtraF32 for f32 {
    fn lerp(from: f32, to: f32, t: f32) -> f32 {
        from + t * (to - from)
    }

    fn step(from: f32, to: f32, step: f32) -> f32 {
        let mut step = step;
        if step < 0.0 {
            step *= -1.0;
            //panic!("f32::step(__, __, `can't be negtive`)");
        }

        let diff = to - from;
        if f32::abs(diff) <= f32::EPSILON {
            to
        } else {
            from + (f32::signum(diff) * step)
        }
    }
}

// }}}

// Vec2 {{{

#[inline(always)]
pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    #[inline(always)]
    pub fn init(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub fn dot(lhs: Self, rhs: Self) -> f32 {
        (lhs.x * rhs.x) + (lhs.y * rhs.y)
    }

    #[inline(always)]
    pub fn cross(lhs: Self, rhs: Self) -> f32 {
        (lhs.x * rhs.y) - (lhs.y * rhs.x)
    }

    #[inline(always)]
    pub fn mag(v: Self) -> f32 {
        f32::sqrt(Self::dot(v, v))
    }

    #[inline(always)]
    pub fn norm(v: Self) -> Self {
        let mag = Self::mag(v);
        if mag <= f32::EPSILON {
            return Self::default();
        }

        Self {
            x: (v.x / mag),
            y: (v.y / mag),
        }
    }

    #[inline(always)]
    pub fn abs(v: Self) -> Self {
        Self {
            x: f32::abs(v.x),
            y: f32::abs(v.y),
        }
    }

    #[inline(always)]
    pub fn lerp(from: Self, to: Self, t: f32) -> Self {
        Self {
            x: f32::lerp(from.x, to.x, t),
            y: f32::lerp(from.y, to.y, t),
        }
    }

    #[inline(always)]
    pub fn step(from: Self, to: Self, step: f32) -> Self {
        Self {
            x: f32::step(from.x, to.x, step),
            y: f32::step(from.y, to.y, step),
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            y: self.y * rhs.y,
            x: self.x * rhs.x,
        }
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            y: self.y * rhs,
            x: self.x * rhs,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            y: self * rhs.y,
            x: self * rhs.x,
        }
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

// }}}

// Vec3 {{{

#[inline(always)]
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    #[inline(always)]
    pub fn init(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline(always)]
    pub fn dot(lhs: Self, rhs: Self) -> f32 {
        (lhs.x * rhs.x) + (lhs.y * rhs.y) + (lhs.z * rhs.z)
    }

    #[inline(always)]
    pub fn cross(lhs: Self, rhs: Self) -> Self {
        Self {
            x: (lhs.y * rhs.z) - (lhs.z * rhs.y),
            y: (lhs.z * rhs.x) - (lhs.x * rhs.z),
            z: (lhs.x * rhs.y) - (lhs.y * rhs.x),
        }
    }

    #[inline(always)]
    pub fn mag(v: Self) -> f32 {
        f32::sqrt(Self::dot(v, v))
    }

    #[inline(always)]
    pub fn norm(v: Self) -> Self {
        let mag = Self::mag(v);
        if mag <= f32::EPSILON {
            return Self::default();
        }

        Self {
            x: (v.x / mag),
            y: (v.y / mag),
            z: (v.z / mag),
        }
    }

    #[inline(always)]
    pub fn abs(v: Self) -> Self {
        Self {
            x: f32::abs(v.x),
            y: f32::abs(v.y),
            z: f32::abs(v.z),
        }
    }

    #[inline(always)]
    pub fn lerp(from: Self, to: Self, t: f32) -> Self {
        Self {
            x: f32::lerp(from.x, to.x, t),
            y: f32::lerp(from.y, to.y, t),
            z: f32::lerp(from.z, to.z, t),
        }
    }

    #[inline(always)]
    pub fn step(from: Self, to: Self, step: f32) -> Self {
        Self {
            x: f32::step(from.x, to.x, step),
            y: f32::step(from.y, to.y, step),
            z: f32::step(from.z, to.z, step),
        }
    }

    pub fn as_ptr(&self) -> *const f32 {
        &self.x as *const f32
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
// }}}

// Vec4 {{{

#[inline(always)]
pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4 { x, y, z, w }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    #[inline(always)]
    pub fn init(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    #[inline(always)]
    pub fn scale(lhs: Self, rhs: f32) -> Self {
        Self {
            x: lhs.x * rhs,
            y: lhs.y * rhs,
            z: lhs.z * rhs,
            w: lhs.w * rhs,
        }
    }

    #[inline(always)]
    pub fn abs(v: Self) -> Self {
        Self {
            x: f32::abs(v.x),
            y: f32::abs(v.y),
            z: f32::abs(v.z),
            w: f32::abs(v.w),
        }
    }

    #[inline(always)]
    pub fn lerp(from: Self, to: Self, t: f32) -> Self {
        Self {
            x: f32::lerp(from.x, to.x, t),
            y: f32::lerp(from.y, to.y, t),
            z: f32::lerp(from.z, to.z, t),
            w: f32::lerp(from.w, to.w, t),
        }
    }

    #[inline(always)]
    pub fn step(from: Self, to: Self, step: f32) -> Self {
        Self {
            x: f32::step(from.x, to.x, step),
            y: f32::step(from.y, to.y, step),
            z: f32::step(from.z, to.z, step),
            w: f32::step(from.w, to.w, step),
        }
    }

    pub fn as_ptr(&self) -> *const f32 {
        &self.x as *const f32
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Vec4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl MulAssign for Vec4 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}

impl Div for Vec4 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}

impl DivAssign for Vec4 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

impl Div<Vec4> for f32 {
    type Output = Vec4;

    fn div(self, rhs: Vec4) -> Self::Output {
        Self::Output {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
            w: self / rhs.w,
        }
    }
}

impl Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

// }}}

// Versor {{{

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Versor {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Versor {
    #[inline(always)]
    pub fn identity() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    #[inline(always)]
    pub fn real(&self) -> f32 {
        self.w
    }

    #[inline(always)]
    pub fn imaginary(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    #[inline(always)]
    pub fn dot(&self, rhs: &Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z) + (self.w * rhs.w)
    }

    #[inline(always)]
    pub fn mag(&self) -> f32 {
        f32::sqrt(Self::dot(self, self))
    }

    #[inline(always)]
    pub fn norm(&self) -> Self {
        let m = Self::mag(self);

        if m <= f32::EPSILON {
            return Self::identity();
        }

        Self {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
            w: self.w / m,
        }
    }

    #[inline(always)]
    pub fn conj(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }

    #[inline(always)]
    pub fn mul(&self, rhs: &Self) -> Self {
        let l_real = Self::real(self);
        let l_img = Self::imaginary(self);

        let r_real = Self::real(rhs);
        let r_img = Self::imaginary(rhs);

        let real = (l_real * r_real) - Vec3::dot(r_img, l_img);

        let a = Vec3::cross(r_img, l_img);
        let b = r_img * l_real;
        let c = l_img * r_real;

        let mut imaginary = Vec3::add(a, b);
        imaginary = Vec3::add(imaginary, c);

        Self {
            x: imaginary.x,
            y: imaginary.y,
            z: imaginary.z,
            w: real,
        }
    }

    #[inline(always)]
    pub fn from_euler(axis: Vec3, angle: f32) -> Self {
        let angle = angle.to_radians();

        Self {
            x: f32::sin(angle / 2.0) * axis.x,
            y: f32::sin(angle / 2.0) * axis.y,
            z: f32::sin(angle / 2.0) * axis.z,
            w: f32::cos(angle / 2.0),
        }
        .norm()
    }

    #[inline(always)]
    pub fn around_axis_degrees(x: f32, y: f32, z: f32) -> Self {
        let vx = Self::from_euler(
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            x,
        );
        let vy = Self::from_euler(
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            y,
        );
        let vz = Self::from_euler(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            z,
        );
        vx.mul(&vy.mul(&vz))
    }
}
// }}}

// Mat4 {{{

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Mat4 {
    pub m: [[f32; 4]; 4],
}

impl Mat4 {
    pub fn identity() -> Self {
        let mut r = Self::default();
        r.m[0][0] = 1.0;
        r.m[1][1] = 1.0;
        r.m[2][2] = 1.0;
        r.m[3][3] = 1.0;
        r
    }

    /// takes in radians
    pub fn rotation(yaw: f32, pitch: f32, roll: f32) -> Self {
        //let cosy, siny, cosb, sinb, cosa, sina;

        let cosy = f32::cos(yaw);
        let siny = f32::sin(yaw);

        let cosb = f32::cos(pitch);
        let sinb = f32::sin(pitch);

        let cosa = f32::cos(roll);
        let sina = f32::sin(roll);

        let mut r: Self = Self::default();
        r.m[0][0] = (cosy * cosb * cosa) - (siny * sina);
        r.m[0][1] = (siny * cosb * cosa) + (cosy * sina);
        r.m[0][2] = -sinb * cosa;
        //r.m[0][3] = 0.0;

        r.m[1][0] = (-cosy * cosb * sina) - (siny * cosa);
        r.m[1][1] = (-siny * cosb * sina) + (cosy * cosa);
        r.m[1][2] = sinb * sina;
        //r.m[1][3] = 0.0;

        r.m[2][0] = cosy * sina;
        r.m[2][1] = siny * sinb;
        r.m[2][2] = cosb;
        //r.m[2][3] = 0.0;

        //r.m[3][0] = 0.0;
        //r.m[3][1] = 0.0;
        //r.m[3][2] = 0.0;
        r.m[3][3] = 1.0;

        r
    }

    #[inline(always)]
    pub fn rotation_deg(yaw: f32, pitch: f32, roll: f32) -> Self {
        Self::rotation(
            f32::to_radians(yaw),
            f32::to_radians(pitch),
            f32::to_radians(roll),
        )
    }

    pub fn translate(mut self, v: Vec3) -> Self {
        self.m[3][0] = v.x;
        self.m[3][1] = v.y;
        self.m[3][2] = v.z;
        self
    }

    pub fn scale(mut self, v: Vec3) -> Self {
        self.m[0][0] *= v.x;
        self.m[1][1] *= v.y;
        self.m[2][2] *= v.z;
        self
    }

    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, n: f32, f: f32) -> Self {
        let mut r = Self::default();

        let right_left = 1.0 / (right - left);
        let top_bot = 1.0 / (top - bottom);
        let far_near = -1.0 / (f - n);

        r.m[0][0] = 2.0 * right_left;
        r.m[1][1] = 2.0 * top_bot;
        r.m[2][2] = -2.0 * far_near;
        r.m[3][0] = -(right + left) * right_left;
        r.m[3][1] = -(top + bottom) * top_bot;
        r.m[3][2] = -(f + n) * far_near;
        r.m[3][3] = 1.0;

        r
    }

    pub fn perspective(fov: f32, aspect: f32, n: f32, f: f32) -> Self {
        let mut r = Self::default();

        let focal = 1.0 / f32::tan(fov / 2.0);
        let near_far = 1.0 / (n - f);

        r.m[0][0] = focal / aspect;
        r.m[1][1] = focal;
        r.m[2][2] = (f + n) * near_far;
        r.m[2][3] = -1.0;
        r.m[3][2] = 2.0 * f * n * near_far;

        r
    }

    pub fn _mul(lhs: Self, rhs: Self) -> Self {
        let a00 = lhs.m[0][0];
        let a01 = lhs.m[0][1];
        let a02 = lhs.m[0][2];
        let a03 = lhs.m[0][3];
        let a10 = lhs.m[1][0];
        let a11 = lhs.m[1][1];
        let a12 = lhs.m[1][2];
        let a13 = lhs.m[1][3];
        let a20 = lhs.m[2][0];
        let a21 = lhs.m[2][1];
        let a22 = lhs.m[2][2];
        let a23 = lhs.m[2][3];
        let a30 = lhs.m[3][0];
        let a31 = lhs.m[3][1];
        let a32 = lhs.m[3][2];
        let a33 = lhs.m[3][3];

        let b00 = rhs.m[0][0];
        let b01 = rhs.m[0][1];
        let b02 = rhs.m[0][2];
        let b10 = rhs.m[1][0];
        let b11 = rhs.m[1][1];
        let b12 = rhs.m[1][2];
        let b20 = rhs.m[2][0];
        let b21 = rhs.m[2][1];
        let b22 = rhs.m[2][2];
        let b30 = rhs.m[3][0];
        let b31 = rhs.m[3][1];
        let b32 = rhs.m[3][2];
        let b33 = rhs.m[3][3];

        let mut r: Self = Self::default();
        r.m[0][0] = a00 * b00 + a10 * b01 + a20 * b02;
        r.m[0][1] = a01 * b00 + a11 * b01 + a21 * b02;
        r.m[0][2] = a02 * b00 + a12 * b01 + a22 * b02;
        r.m[0][3] = a03 * b00 + a13 * b01 + a23 * b02;

        r.m[1][0] = a00 * b10 + a10 * b11 + a20 * b12;
        r.m[1][1] = a01 * b10 + a11 * b11 + a21 * b12;
        r.m[1][2] = a02 * b10 + a12 * b11 + a22 * b12;
        r.m[1][3] = a03 * b10 + a13 * b11 + a23 * b12;

        r.m[2][0] = a00 * b20 + a10 * b21 + a20 * b22;
        r.m[2][1] = a01 * b20 + a11 * b21 + a21 * b22;
        r.m[2][2] = a02 * b20 + a12 * b21 + a22 * b22;
        r.m[2][3] = a03 * b20 + a13 * b21 + a23 * b22;

        r.m[3][0] = a00 * b30 + a10 * b31 + a20 * b32 + a30 * b33;
        r.m[3][1] = a01 * b30 + a11 * b31 + a21 * b32 + a31 * b33;
        r.m[3][2] = a02 * b30 + a12 * b31 + a22 * b32 + a32 * b33;
        r.m[3][3] = a03 * b30 + a13 * b31 + a23 * b32 + a33 * b33;

        r
    }

    pub fn from_versor(v: Versor) -> Self {
        let mag = v.mag();
        let s = if mag > 0.0 { 2.0 / mag } else { 0.0 };

        let xx = s * v.x * v.x;
        let yy = s * v.y * v.y;
        let zz = s * v.z * v.z;

        let xy = s * v.x * v.y;
        let yz = s * v.y * v.z;
        let xz = s * v.x * v.z;

        let wx = s * v.w * v.x;
        let wy = s * v.w * v.y;
        let wz = s * v.w * v.z;

        let mut r: Self = Self::default();

        r.m[0][0] = 1.0 - yy - zz;
        r.m[0][1] = xy + wz;
        r.m[0][2] = xz - wy;
        r.m[0][3] = 0.0;

        r.m[1][0] = xy - wz;
        r.m[1][1] = 1.0 - xx - zz;
        r.m[1][2] = yz + wx;
        r.m[1][3] = 0.0;

        r.m[2][0] = xz + wy;
        r.m[2][1] = yz - wx;
        r.m[2][2] = 1.0 - xx - yy;
        r.m[2][3] = 0.0;

        r.m[3][0] = 0.0;
        r.m[3][1] = 0.0;
        r.m[3][2] = 0.0;
        r.m[3][3] = 1.0;

        r
    }

    pub fn transform(scale: Vec3, rotation: Versor, translate: Vec3) -> Self {
        let s = Self::identity().scale(scale);
        let r = Self::from_versor(rotation);
        let t = Self::mul(s, r);
        t.translate(translate)
    }

    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let f = Vec3::sub(target, eye);
        let f = Vec3::norm(f);

        let s = Vec3::cross(f, Vec3::norm(up));
        let s = Vec3::norm(s);

        let u = Vec3::cross(s, f);

        let mut result: Self = Self::default();
        result.m[0][0] = s.x;
        result.m[0][1] = u.x;
        result.m[0][2] = -f.x;

        result.m[1][0] = s.y;
        result.m[1][1] = u.y;
        result.m[1][2] = -f.y;

        result.m[2][0] = s.z;
        result.m[2][1] = u.z;
        result.m[2][2] = -f.z;

        result.m[3][0] = -Vec3::dot(s, eye);
        result.m[3][1] = -Vec3::dot(u, eye);
        result.m[3][2] = Vec3::dot(f, eye);

        result.m[3][3] = 1.0;

        //result.m[0][3] = 0.0;
        //result.m[1][3] = 0.0;
        //result.m[2][3] = 0.0;
        result
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.m[0].as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut f32 {
        self.m[0].as_mut_ptr()
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::_mul(self, rhs)
    }
}

// }}}

// Vec3i {{{

#[inline(always)]
pub fn ivec3(x: i32, y: i32, z: i32) -> Vec3i {
    Vec3i { x, y, z }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct Vec3i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3i {
    #[inline(always)]
    pub fn init(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    #[inline(always)]
    pub fn dot(lhs: Self, rhs: Self) -> i32 {
        (lhs.x * rhs.x) + (lhs.y * rhs.y) + (lhs.z * rhs.z)
    }

    #[inline(always)]
    pub fn cross(lhs: Self, rhs: Self) -> Self {
        Self {
            x: (lhs.y * rhs.z) - (lhs.z * rhs.y),
            y: (lhs.z * rhs.x) - (lhs.x * rhs.z),
            z: (lhs.x * rhs.y) - (lhs.y * rhs.x),
        }
    }

    #[inline(always)]
    pub fn abs(v: Self) -> Self {
        Self {
            x: i32::abs(v.x),
            y: i32::abs(v.y),
            z: i32::abs(v.z),
        }
    }

    pub fn as_ptr(&self) -> *const i32 {
        &self.x as *const i32
    }
}

impl Add for Vec3i {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3i {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3i {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3i {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Vec3i {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign for Vec3i {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Mul<i32> for Vec3i {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3i> for i32 {
    type Output = Vec3i;

    fn mul(self, rhs: Vec3i) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div for Vec3i {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign for Vec3i {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Div<i32> for Vec3i {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<i32> for Vec3i {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Div<Vec3i> for i32 {
    type Output = Vec3i;

    fn div(self, rhs: Vec3i) -> Self::Output {
        Self::Output {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

impl Neg for Vec3i {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// }}}

#[cfg(test)]
mod tests;
