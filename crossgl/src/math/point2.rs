use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::{Point2, Size2, Vec2};
use crate::num::{Max, Min, NegOne, One, Zero};

impl<T> Point2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub const fn splat(v: T) -> Self
    where
        T: Copy,
    {
        Self::new(v, v)
    }

    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Point2<U> {
        Point2::new(f(self.x), f(self.y))
    }

    pub const fn to_array(self) -> [T; 2] {
        unsafe { transmute!(for<T> Point2<T>, [T; 2], self) }
    }
}

impl<T: Zero> Point2<T> {
    pub const ZERO: Self = Self::splat(T::ZERO);
}

impl<T: One> Point2<T> {
    pub const ONE: Self = Self::splat(T::ONE);
}

impl<T: NegOne> Point2<T> {
    pub const NEG_ONE: Self = Self::splat(T::NEG_ONE);
}

impl<T: Max> Point2<T> {
    pub const MAX: Self = Self::splat(T::MAX);
}

impl<T: Min> Point2<T> {
    pub const MIN: Self = Self::splat(T::MIN);
}

impl<T> From<Vec2<T>> for Point2<T> {
    fn from(Vec2 { x, y }: Vec2<T>) -> Self {
        Self { x, y }
    }
}

impl<T> From<(T, T)> for Point2<T> {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

impl<T> From<[T; 2]> for Point2<T> {
    fn from([x, y]: [T; 2]) -> Self {
        Self::new(x, y)
    }
}

impl<T> From<Point2<T>> for (T, T) {
    fn from(Point2 { x, y }: Point2<T>) -> Self {
        (x, y)
    }
}

impl<T> From<Point2<T>> for [T; 2] {
    fn from(Point2 { x, y }: Point2<T>) -> Self {
        [x, y]
    }
}

impl<T> Add for Point2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> AddAssign for Point2<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub for Point2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> SubAssign for Point2<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Mul for Point2<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T> MulAssign for Point2<T>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T> Div for Point2<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<T> DivAssign for Point2<T>
where
    T: DivAssign,
{
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

// `Vec2`

impl<T> Add<Vec2<T>> for Point2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> AddAssign<Vec2<T>> for Point2<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Vec2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub<Vec2<T>> for Point2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> SubAssign<Vec2<T>> for Point2<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Mul<Vec2<T>> for Point2<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T> MulAssign<Vec2<T>> for Point2<T>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, rhs: Vec2<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T> Div<Vec2<T>> for Point2<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Vec2<T>) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<T> DivAssign<Vec2<T>> for Point2<T>
where
    T: DivAssign,
{
    fn div_assign(&mut self, rhs: Vec2<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

// `(T, T)`

impl<T> Add<(T, T)> for Point2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: (T, T)) -> Self::Output {
        Self::new(self.x + rhs.0, self.y + rhs.1)
    }
}

impl<T> AddAssign<(T, T)> for Point2<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: (T, T)) {
        self.x += rhs.0;
        self.y += rhs.1;
    }
}

impl<T> Sub<(T, T)> for Point2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: (T, T)) -> Self::Output {
        Self::new(self.x - rhs.0, self.y - rhs.1)
    }
}

impl<T> SubAssign<(T, T)> for Point2<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: (T, T)) {
        self.x -= rhs.0;
        self.y -= rhs.1;
    }
}

impl<T> Mul<(T, T)> for Point2<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: (T, T)) -> Self::Output {
        Self::new(self.x * rhs.0, self.y * rhs.1)
    }
}

impl<T> MulAssign<(T, T)> for Point2<T>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, rhs: (T, T)) {
        self.x *= rhs.0;
        self.y *= rhs.1;
    }
}

impl<T> Div<(T, T)> for Point2<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: (T, T)) -> Self::Output {
        Self::new(self.x / rhs.0, self.y / rhs.1)
    }
}

impl<T> DivAssign<(T, T)> for Point2<T>
where
    T: DivAssign,
{
    fn div_assign(&mut self, rhs: (T, T)) {
        self.x /= rhs.0;
        self.y /= rhs.1;
    }
}

// `Size2<T>`

impl<T> Add<Size2<T>> for Point2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Size2<T>) -> Self::Output {
        Self::new(self.x + rhs.w, self.y + rhs.h)
    }
}

impl<T> AddAssign<Size2<T>> for Point2<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Size2<T>) {
        self.x += rhs.w;
        self.y += rhs.h;
    }
}

impl<T> Sub<Size2<T>> for Point2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Size2<T>) -> Self::Output {
        Self::new(self.x - rhs.w, self.y - rhs.h)
    }
}

impl<T> SubAssign<Size2<T>> for Point2<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Size2<T>) {
        self.x -= rhs.w;
        self.y -= rhs.h;
    }
}

impl<T> Mul<Size2<T>> for Point2<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Size2<T>) -> Self::Output {
        Self::new(self.x * rhs.w, self.y * rhs.h)
    }
}

impl<T> MulAssign<Size2<T>> for Point2<T>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, rhs: Size2<T>) {
        self.x *= rhs.w;
        self.y *= rhs.h;
    }
}

impl<T> Div<Size2<T>> for Point2<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Size2<T>) -> Self::Output {
        Self::new(self.x / rhs.w, self.y / rhs.h)
    }
}

impl<T> DivAssign<Size2<T>> for Point2<T>
where
    T: DivAssign,
{
    fn div_assign(&mut self, rhs: Size2<T>) {
        self.x /= rhs.w;
        self.y /= rhs.h;
    }
}
// TODO: ops for `Size2` and `Vec2`
