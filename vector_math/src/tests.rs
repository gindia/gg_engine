#![allow(unused_imports)]

use super::*;

#[test]
fn f32_extra_step_test() {
    {
        let to = 5.0;
        let step = 1.0;
        let mut from = 0.0;
        from = f32::step(from, to, step);
        assert_eq!(from, 1.0);
        from = f32::step(from, to, step);
        assert_eq!(from, 2.0);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        assert_eq!(from, to);
    }

    {
        let to = -5.0;
        let step = 1.0;
        let mut from = 0.0;
        from = f32::step(from, to, step);
        assert_eq!(from, -1.0);
        from = f32::step(from, to, step);
        assert_eq!(from, -2.0);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        assert_eq!(from, to);
    }

    {
        let to = -10.0;
        let step = 0.5;
        let mut from = -5.0;
        from = f32::step(from, to, step);
        assert_eq!(from, -5.5);
        from = f32::step(from, to, step);
        assert_eq!(from, -6.0);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        assert_eq!(from, to);
    }

    {
        let to = -10.0;
        let step = -0.5;
        let mut from = -5.0;
        from = f32::step(from, to, step);
        assert_eq!(from, -5.5);
        from = f32::step(from, to, step);
        assert_eq!(from, -6.0);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        from = f32::step(from, to, step);
        assert_eq!(from, to);
    }
}

#[test]
fn m4_tests() {
    {
        let m0 = Mat4::default();
        let m1 = Mat4 { m: [[0.0; 4]; 4] };
        assert_eq!(m0, m1);
    }

    {
        let m0 = Mat4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        let m1 = Mat4::identity();
        assert_eq!(m0, m1);
    }

    {
        let m0 = Mat4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [3.0, 3.0, 3.0, 1.0],
            ],
        };

        let m1 = Mat4::identity().translate(Vec3::init(3.0, 3.0, 3.0));
        assert_eq!(m0, m1);
    }

    {
        let m0 = Mat4 {
            m: [
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 2.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        let m1 = Mat4::identity().scale(Vec3::init(2.0, 2.0, 2.0));
        assert_eq!(m0, m1);
    }

}
