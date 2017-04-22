use std::ops::*;

pub trait Dot<RHS=Self>
{
    type Output;

    fn dot(self, rhs: RHS) -> Self::Output;
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Vector2<T: Copy>
{
    pub components: [T; 2]
}

impl Vector2<f32>
{
    pub fn round_i32(self) -> Vector2<i32>
    {
        vec2(self.components[0].round() as i32, self.components[1].round() as i32)
    }
}

pub fn vec2<T: Copy>(x: T, y: T) -> Vector2<T>
{
    Vector2 { components: [x, y]}
}


impl<T> Add for Vector2<T>
    where T: Copy + Add<Output=T>
{
    type Output = Self;

    fn add(self, other: Self) -> Self
    {
        let mut out = self;
        for (i, c) in other.components.iter().enumerate()
        {
            out.components[i] = out.components[i] + *c;
        }
        out
    }
}

impl<T> Sub for Vector2<T>
    where T: Copy + Sub<Output=T>
{
    type Output = Self;

    fn sub(self, other: Self) -> Self
    {
        let mut out = self;
        for (i, c) in other.components.iter().enumerate()
        {
            out.components[i] = out.components[i] - *c;
        }
        out
    }
}

impl<T> Mul<T> for Vector2<T>
    where T: Copy + Mul<Output=T>
{
    type Output = Self;

    fn mul(self, other: T) -> Self
    {
        let mut out = self;
        for c in out.components.iter_mut()
        {
            *c = *c * other;
        }
        out
    }
}

impl<T> Dot for Vector2<T>
    where T: Copy + Default + Add<Output=T> + Mul<Output=T>
{
    type Output = T;

    fn dot(self, other: Self) -> T
    {
        let mut accum = T::default();
        for (i, c) in self.components.iter().enumerate()
        {
            accum = accum + (*c * other.components[i]);
        }
        accum
    }
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    pub fn vector_operators()
    {
        let u = vec2(40, 20);
        let v = vec2(10, 15);
        assert_eq!(u + v, vec2(50, 35));
        assert_eq!(u - v, vec2(30, 5));
        assert_eq!(u * 10, vec2(400, 200));
    }

    #[test]
    pub fn dot_product()
    {
        let a = vec2(1, 0);
        let b = vec2(0, 1);
        assert_eq!(a.dot(b), 0);
        assert_eq!(a.dot(a), 1);
    }
}
