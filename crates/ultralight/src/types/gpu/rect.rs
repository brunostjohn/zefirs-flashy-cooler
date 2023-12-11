use ultralight_sys::{ULIntRect, ULRect};

#[derive(Debug)]
pub struct Rect<T> {
    pub left: T,
    pub top: T,
    pub right: T,
    pub bottom: T,
}

impl Rect<i32> {
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.left == 0 && self.top == 0 && self.right == 0 && self.bottom == 0
    }
}

impl From<ULRect> for Rect<f32> {
    #[inline(always)]
    fn from(r: ULRect) -> Self {
        Rect {
            left: r.left,
            top: r.top,
            right: r.right,
            bottom: r.bottom,
        }
    }
}

impl From<ULIntRect> for Rect<i32> {
    #[inline(always)]
    fn from(r: ULIntRect) -> Self {
        Rect {
            left: r.left,
            top: r.top,
            right: r.right,
            bottom: r.bottom,
        }
    }
}

impl From<Rect<f32>> for ULRect {
    #[inline(always)]
    fn from(r: Rect<f32>) -> Self {
        ULRect {
            left: r.left,
            top: r.top,
            right: r.right,
            bottom: r.bottom,
        }
    }
}

impl From<Rect<i32>> for ULIntRect {
    #[inline(always)]
    fn from(r: Rect<i32>) -> Self {
        ULIntRect {
            left: r.left,
            top: r.top,
            right: r.right,
            bottom: r.bottom,
        }
    }
}
