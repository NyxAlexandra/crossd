use std::fmt;
use std::mem::ManuallyDrop;

use bytemuck::{Pod, Zeroable};
use mint::IntoMint;

use super::{Mat4, One, Vec4, Zero};

impl<T: fmt::Display + Copy> fmt::Display for Mat4<T> {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // without delimeters for columns so as to not be confusing to read.
        // 
        // ```
        // [ 1 0 0 0
        //   0 1 0 0
        //   0 0 1 0
        //   0 0 0 1 ]
        // ```
        // 
        // ```
        // [ 1 2 3 4
        //   5 6 7 8
        //   9 10 11 12
        //   13 14 15 16 ]
        // ```

        let trans = self.transposed();

        write!(
            f,
            "[ {} {} {} {}\n  {} {} {} {}\n  {} {} {} {}\n  {} {} {} {} ]",
            trans.x.x, trans.x.y, trans.x.z, trans.x.w,
            trans.y.x, trans.y.y, trans.y.z, trans.y.w,
            trans.z.x, trans.z.y, trans.z.z, trans.z.w,
            trans.w.x, trans.w.y, trans.w.z, trans.w.w,
        )
    }
}

/// A union for casting between different representations of a [`Mat4`].
#[repr(C)]
union Cast<T> {
    mat4: ManuallyDrop<Mat4<T>>,

    vecs: ManuallyDrop<[Vec4<T>; 4]>,
    arrays: ManuallyDrop<[[T; 4]; 4]>,
    array: ManuallyDrop<[T; 16]>,
    mint: ManuallyDrop<mint::ColumnMatrix4<T>>,
}

impl<T> Mat4<T> {
    /// Create a new matrix from column arrays.
    ///
    /// ```
    /// # use crossd_graphics::math::Mat4;
    /// #
    /// let x = [1, 0, 0, 0];
    /// let y = [0, 1, 0, 0];
    /// let z = [0, 0, 1, 0];
    /// let w = [0, 0, 0, 1];
    ///
    /// assert_eq!(Mat4::new(x, y, z, w), Mat4::IDENTITY);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(x: [T; 4], y: [T; 4], z: [T; 4], w: [T; 4]) -> Self {
        ManuallyDrop::into_inner(unsafe {
            Cast { arrays: ManuallyDrop::new([x, y, z, w]) }.mat4
        })
    }

    /// Construct a new matrix from it's column vectors.
    #[inline]
    #[must_use]
    pub const fn new_vecs(x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T>) -> Self {
        Self { x, y, z, w }
    }

    /// Create a new matrix from it's columns.
    ///
    /// ```
    /// # use crossd_graphics::math::{Mat4, vec4};
    /// #
    /// let x = vec4(1, 0, 0, 0);
    /// let y = vec4(0, 1, 0, 0);
    /// let z = vec4(0, 0, 1, 0);
    /// let w = vec4(0, 0, 0, 1);
    ///
    /// assert_eq!(Mat4::from_vecs([x, y, z, w]), Mat4 { x, y, z, w });
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_vecs(vec4s: [Vec4<T>; 4]) -> Self {
        ManuallyDrop::into_inner(unsafe { Cast { vecs: ManuallyDrop::new(vec4s) }.mat4 })
    }

    /// Convert this matrix to an array of columns.
    #[inline]
    #[must_use]
    pub const fn to_vecs(self) -> [Vec4<T>; 4] {
        ManuallyDrop::into_inner(unsafe { Cast { mat4: ManuallyDrop::new(self) }.vecs })
    }

    /// Create a new matrix from the components of each column in order.
    #[inline]
    #[must_use]
    pub const fn from_array(array: [T; 16]) -> Self {
        ManuallyDrop::into_inner(unsafe { Cast { array: ManuallyDrop::new(array) }.mat4 })
    }

    /// Convert to an array of components of each column.
    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [T; 16] {
        ManuallyDrop::into_inner(unsafe { Cast { mat4: ManuallyDrop::new(self) }.array })
    }

    /// Transform a `Mat4<T>` -> `Mat4<U>` with a callback called on each
    /// column vector.
    #[inline]
    #[must_use]
    pub fn map<U>(self, f: impl FnMut(Vec4<T>) -> Vec4<U>) -> Mat4<U> {
        Mat4::from_vecs(self.to_vecs().map(f))
    }

    /// Transform a `Mat4<T>` -> `Mat4<U>` with a callback called on each
    /// component.
    #[inline]
    #[must_use]
    pub fn each<U>(self, f: impl FnMut(T) -> U) -> Mat4<U> {
        Mat4::from_array(self.to_array().map(f))
    }

    /// Treat the matrix like a slice array of columns.
    #[inline]
    #[must_use]
    pub const fn as_vecs(&self) -> &[Vec4<T>] {
        unsafe { &*(self as *const Self as *const [Vec4<T>; 4] as *const _) }
    }

    /// Treat the matrix like a slice array of columns.
    #[inline]
    #[must_use]
    pub const fn as_arrays(&self) -> &[[T; 4]] {
        unsafe { &*(self as *const Self as *const [[T; 4]; 4] as *const _) }
    }

    /// Treat the matrix like a slice array of components.
    #[inline]
    #[must_use]
    pub const fn as_slice(&self) -> &[T] {
        unsafe { &*(self as *const Self as *const [T; 16] as *const _) }
    }
}

impl<T: Copy> Mat4<T> {
    /// Create a new matrix from it's row arrays.
    #[inline]
    #[must_use]
    pub const fn new_rows(x: [T; 4], y: [T; 4], z: [T; 4], w: [T; 4]) -> Self {
        Self::new(x, y, z, w).transposed()
    }

    /// Create a new matrix from it's row vectors.
    #[inline]
    #[must_use]
    pub const fn new_vec_rows(x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T>) -> Self {
        Self::new_vecs(x, y, z, w).transposed()
    }

    /// Create a new matrix using `v` as each component.
    ///
    /// ```
    /// # use crossd_graphics::math::{Mat4, Vec4};
    /// #
    /// let v = 7;
    /// let mat4 = Mat4::splat(v);
    ///
    /// assert_eq!(mat4, Mat4::from_array([v; 16]));
    /// assert_eq!(mat4, Mat4::from_vecs([Vec4::splat(v); 4]));
    /// ```
    #[inline]
    #[must_use]
    pub const fn splat(v: T) -> Self {
        Self::from_array([v; 16])
    }

    /// Create a new matrix using `v` on the diagonal and `o` for the rest.
    #[inline]
    #[must_use]
    pub const fn with_diagonal_or(v: T, o: T) -> Self {
        Self::new([v, o, o, o], [o, v, o, o], [o, o, v, o], [o, o, o, v])
    }

    /// ## Panics
    ///
    /// Panics if the index is not of `0..4`.
    #[inline]
    #[must_use]
    pub const fn get(self, n: usize) -> Vec4<T> {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            3 => self.w,
            _ => panic!(),
        }
    }

    /// Get the element at the coordinate.
    ///
    /// ```
    /// # use crossd_graphics::math::Mat4;
    /// #
    /// let mat4: Mat4<i32> = Mat4::IDENTITY;
    ///
    /// assert_eq!(mat4.at(0, 0), 1);
    /// ```
    ///
    /// ## Panics
    ///
    /// Panics if either index is not of `0..4`.
    #[inline]
    #[must_use]
    pub const fn at(self, n: usize, m: usize) -> T {
        self.get(n).get(m)
    }

    /// Transpose the matrix (like flipping over the diagonal).
    ///
    /// ```
    /// # use crossd_graphics::math::Mat4;
    /// #
    /// let x = [1, 2, 3, 4];
    /// let y = [5, 6, 7, 8];
    /// let z = [9, 10, 11, 12];
    /// let w = [13, 14, 15, 16];
    ///
    /// let col = Mat4::new(x, y, z, w);
    /// let row = Mat4::new_rows(x, y, z, w);
    ///
    /// assert_eq!(col.transposed(), row);
    /// assert_eq!(row.transposed(), col);
    /// assert_eq!(col.transposed().transposed(), col);
    /// ```
    #[inline]
    #[must_use]
    pub const fn transposed(self) -> Self {
        // this could probably be done without `T: Copy`, but I don't care enough (for
        // now)
        Self::new(
            [self.x.x, self.y.x, self.z.x, self.w.x],
            [self.x.y, self.y.y, self.z.y, self.w.y],
            [self.x.z, self.y.z, self.z.z, self.w.z],
            [self.x.w, self.y.w, self.z.w, self.w.w],
        )
    }

    /// The diagonal of the matrix.
    ///
    /// ```
    /// # use crossd_graphics::math::Mat4;
    /// #
    /// assert_eq!(Mat4::<f32>::IDENTITY.diagonal(), [1.0; 4]);
    /// ```
    #[inline]
    #[must_use]
    pub const fn diagonal(self) -> [T; 4] {
        [self.x.x, self.y.y, self.z.z, self.w.w]
    }
}

impl<T: One> Mat4<T> {
    /// A matrix with each component set to `T::ONE`.
    pub const ONE: Self = Self::splat(T::ONE);
}

impl<T: Zero> Mat4<T> {
    /// A matrix with each component set to `T::ZERO`.
    pub const ZERO: Self = Self::splat(T::ZERO);

    /// Create a new matrix using `v` for the diagonal and `T::ZERO` for the
    /// rest.
    #[inline]
    #[must_use]
    pub const fn with_diagonal(v: T) -> Self {
        Self::with_diagonal_or(v, T::ZERO)
    }
}

impl<T: One + Zero> Mat4<T> {
    /// The identity matrix.
    pub const IDENTITY: Self = Self::with_diagonal(T::ONE);
}

// -----------
// conversions
// -----------

impl<T> AsRef<[Vec4<T>]> for Mat4<T> {
    fn as_ref(&self) -> &[Vec4<T>] {
        self.as_vecs()
    }
}

impl<T> AsRef<[[T; 4]]> for Mat4<T> {
    fn as_ref(&self) -> &[[T; 4]] {
        self.as_arrays()
    }
}

impl<T> AsRef<[T]> for Mat4<T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> IntoMint for Mat4<T> {
    type MintType = mint::ColumnMatrix4<T>;
}

impl<T> From<Mat4<T>> for mint::ColumnMatrix4<T> {
    fn from(mat4: Mat4<T>) -> Self {
        ManuallyDrop::into_inner(unsafe { Cast { mat4: ManuallyDrop::new(mat4) }.mint })
    }
}

impl<T> From<mint::ColumnMatrix4<T>> for Mat4<T> {
    fn from(mint: mint::ColumnMatrix4<T>) -> Self {
        ManuallyDrop::into_inner(unsafe { Cast { mint: ManuallyDrop::new(mint) }.mat4 })
    }
}

unsafe impl<T: Pod> Pod for Mat4<T> {}
unsafe impl<T: Zeroable> Zeroable for Mat4<T> {}

#[cfg(test)]
mod tests {
    use super::super::Mat4;

    #[test]
    fn fmt() {
        let mat4 =
            Mat4::new_rows([1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]);
        let string = "[ 1 2 3 4\n  5 6 7 8\n  9 10 11 12\n  13 14 15 16 ]";

        assert_eq!(mat4.to_string(), string);
    }

    #[test]
    fn to_from() {
        let array = [0; 16];

        assert_eq!(
            Mat4::from_array(array).to_array(),
            array,
            "arrays and matrices should have the same memory layout",
        );
    }

    #[test]
    fn identity() {
        assert_eq!(
            Mat4::<f32>::IDENTITY.transposed(),
            Mat4::IDENTITY,
            "transposing the identity matrix should always equal itself",
        );
    }

    #[test]
    fn mint() {
        let x = [1, 0, 0, 0];
        let y = [0, 1, 0, 0];
        let z = [0, 0, 1, 0];
        let w = [0, 0, 0, 1];

        let mint = mint::ColumnMatrix4 {
            x: mint::Vector4::from(x),
            y: mint::Vector4::from(y),
            z: mint::Vector4::from(z),
            w: mint::Vector4::from(w),
        };
        let mat4 = Mat4::new(x, y, z, w);

        assert_eq!(mint::ColumnMatrix4::from(mat4), mint);
        assert_eq!(Mat4::from(mint), mat4);
    }
}
