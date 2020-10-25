#[derive(Clone, Copy)]
pub struct SizeBase<T>
where
    T: Copy,
{
    w: T,
    h: T,
}
impl<T> SizeBase<T>
where
    T: Copy,
{
    pub fn new(w: T, h: T) -> Self {
        SizeBase::<T> { w, h }
    }
    pub fn width(&self) -> T {
        self.w
    }
    pub fn height(&self) -> T {
        self.h
    }
}
pub type Size = SizeBase<i32>;
pub type SizeF = SizeBase<f32>;

#[derive(Clone, Copy)]
pub struct PointBase<T>
where
    T: Copy,
{
    x: T,
    y: T,
}
impl<T> PointBase<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T) -> Self {
        PointBase::<T> { x, y }
    }
    pub fn x(&self) -> T {
        self.x
    }
    pub fn y(&self) -> T {
        self.y
    }
}
pub type Point = PointBase<i32>;
pub type PointF = PointBase<f32>;

#[derive(Clone, Copy)]
pub struct RectBase<T>
where
    T: Copy,
{
    location: PointBase<T>,
    size: SizeBase<T>,
}
impl<T> RectBase<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T, w: T, h: T) -> Self {
        RectBase::<T> {
            location: PointBase::<T>::new(x, y),
            size: SizeBase::<T>::new(w, h),
        }
    }
    pub fn location(&self) -> PointBase<T> {
        self.location
    }
    pub fn size(&self) -> SizeBase<T> {
        self.size
    }
}
pub type Rect = RectBase<i32>;
pub type RectF = RectBase<f32>;
