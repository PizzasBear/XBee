use core::{array, marker::PhantomData, ops, slice};
use heapless::{String, Vec};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Endianness {
    LittleEndian,
    BigEndian,
}

// 0x04030201 32bit => LE: 1 2 3 4
//                     BE: 4 3 2 1

pub trait WriteStream {
    fn endianness(&self) -> Endianness;
    fn write(&mut self, bytes: &[u8]);
}

pub trait ReadStream {
    fn endianness(&self) -> Endianness;
    fn size(&self) -> usize;
    fn read(&mut self, bytes: &mut [u8]);
}

pub trait InnerData: Sized {
    const MAX_SIZE: Option<usize>;
    const MIN_SIZE: usize;

    fn byte_size(&self) -> usize;
    fn write<T: WriteStream>(&self, stream: &mut T);
    fn read<T: ReadStream>(stream: &mut T, max_size: usize) -> Self;
}

impl<'a, T: ReadStream> ReadStream for &'a mut T {
    #[inline]
    fn endianness(&self) -> Endianness {
        T::endianness(self)
    }
    #[inline]
    fn size(&self) -> usize {
        T::size(self)
    }
    #[inline]
    fn read(&mut self, bytes: &mut [u8]) {
        T::read(self, bytes)
    }
}

impl<'a, T: WriteStream> WriteStream for &'a mut T {
    #[inline]
    fn endianness(&self) -> Endianness {
        T::endianness(self)
    }
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        T::write(self, bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OverwriteLittleEndian<T>(T);

impl<T> ops::Deref for OverwriteLittleEndian<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::DerefMut for OverwriteLittleEndian<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: ReadStream> ReadStream for OverwriteLittleEndian<T> {
    fn endianness(&self) -> Endianness {
        Endianness::LittleEndian
    }
    fn size(&self) -> usize {
        self.0.size()
    }
    fn read(&mut self, bytes: &mut [u8]) {
        self.0.read(bytes);
    }
}

impl<T: WriteStream> WriteStream for OverwriteLittleEndian<T> {
    fn endianness(&self) -> Endianness {
        Endianness::LittleEndian
    }
    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }
}

impl<T: InnerData> InnerData for OverwriteLittleEndian<T> {
    const MAX_SIZE: Option<usize> = T::MAX_SIZE;
    const MIN_SIZE: usize = T::MIN_SIZE;

    fn byte_size(&self) -> usize {
        self.0.byte_size()
    }
    fn read<S: ReadStream>(stream: &mut S, max_size: usize) -> Self {
        Self(T::read(&mut OverwriteLittleEndian(stream), max_size))
    }
    fn write<S: WriteStream>(&self, stream: &mut S) {
        T::write(self, &mut OverwriteLittleEndian(stream));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OverwriteBigEndian<T>(T);

impl<T> ops::Deref for OverwriteBigEndian<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::DerefMut for OverwriteBigEndian<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: ReadStream> ReadStream for OverwriteBigEndian<T> {
    fn endianness(&self) -> Endianness {
        Endianness::BigEndian
    }
    fn size(&self) -> usize {
        self.0.size()
    }
    fn read(&mut self, bytes: &mut [u8]) {
        self.0.read(bytes);
    }
}

impl<T: WriteStream> WriteStream for OverwriteBigEndian<T> {
    fn endianness(&self) -> Endianness {
        Endianness::BigEndian
    }
    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }
}

impl<T: InnerData> InnerData for OverwriteBigEndian<T> {
    const MAX_SIZE: Option<usize> = T::MAX_SIZE;
    const MIN_SIZE: usize = T::MIN_SIZE;

    fn byte_size(&self) -> usize {
        self.0.byte_size()
    }
    fn read<S: ReadStream>(stream: &mut S, max_size: usize) -> Self {
        Self(T::read(&mut OverwriteBigEndian(stream), max_size))
    }
    fn write<S: WriteStream>(&self, stream: &mut S) {
        T::write(self, &mut OverwriteBigEndian(stream));
    }
}

macro_rules! impl_writable {
    ($ty:ty) => {
        impl InnerData for $ty {
            const MAX_SIZE: Option<usize> = Some(Self::MIN_SIZE);
            const MIN_SIZE: usize = Self::BITS as usize / 8;

            fn byte_size(&self) -> usize {
                Self::MIN_SIZE
            }
            fn write<T: WriteStream>(&self, stream: &mut T) {
                let bytes = match stream.endianness() {
                    Endianness::LittleEndian => self.to_le_bytes(),
                    Endianness::BigEndian => self.to_be_bytes(),
                };
                stream.write(&bytes);
            }
            fn read<T: ReadStream>(stream: &mut T, max_size: usize) -> Self {
                assert!(
                    Self::MIN_SIZE <= max_size,
                    "Called `InnerData::read` with `max_size` that is less than the minimum `InnerData::MIN_SIZE`",
                );
                let mut bytes = [0; Self::MIN_SIZE];
                stream.read(&mut bytes);
                match stream.endianness() {
                    Endianness::LittleEndian => Self::from_le_bytes(bytes),
                    Endianness::BigEndian => Self::from_be_bytes(bytes),
                }
            }
        }
    };
}

impl_writable!(u8);
impl_writable!(u16);
impl_writable!(u32);
impl_writable!(u64);
impl_writable!(u128);
impl_writable!(usize);

impl_writable!(i8);
impl_writable!(i16);
impl_writable!(i32);
impl_writable!(i64);
impl_writable!(i128);
impl_writable!(isize);

impl InnerData for bool {
    const MAX_SIZE: Option<usize> = Some(1);
    const MIN_SIZE: usize = 1;

    fn byte_size(&self) -> usize {
        Self::MIN_SIZE
    }
    fn write<T: WriteStream>(&self, stream: &mut T) {
        stream.write(&[*self as u8])
    }
    fn read<T: ReadStream>(stream: &mut T, max_size: usize) -> Self {
        assert!(
            0 < max_size,
            "Called `InnerData::read` with `max_size` that is less than the minimum `InnerData::MIN_SIZE`",
        );
        let mut byte = 0;
        stream.read(slice::from_mut(&mut byte));
        byte != 0
    }
}

impl<T: InnerData, const N: usize> InnerData for [T; N] {
    const MAX_SIZE: Option<usize> = match T::MAX_SIZE {
        Some(max_size) => Some(max_size * N),
        None => None,
    };
    const MIN_SIZE: usize = N * T::MIN_SIZE;

    fn byte_size(&self) -> usize {
        self.iter().map(InnerData::byte_size).sum()
    }
    fn write<S: WriteStream>(&self, stream: &mut S) {
        for x in self {
            x.write(stream);
        }
    }
    fn read<S: ReadStream>(stream: &mut S, max_size: usize) -> Self {
        assert!(
            Self::MIN_SIZE < max_size,
            "Called `InnerData::read` with `max_size` that is less than the minimum `InnerData::MIN_SIZE`",
        );
        let mut field_size =
            Self::MAX_SIZE.map_or(max_size, |c_max_size| c_max_size.min(max_size)) - Self::MIN_SIZE;
        array::from_fn::<T, N, _>(|_| {
            field_size += T::MIN_SIZE;
            let value = T::read(stream, field_size);
            field_size -= value.byte_size();
            value
        })
    }
}

macro_rules! num_len {
    ($name:ident($int:ident)) => {
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, InnerData)]
        #[repr(transparent)]
        pub struct $name($int);

        impl From<$int> for $name {
            fn from(len: $int) -> Self {
                Self(len)
            }
        }
        impl Into<$int> for $name {
            fn into(self) -> $int {
                self.0
            }
        }
        impl From<usize> for $name {
            fn from(len: usize) -> Self {
                Self(len as _)
            }
        }
        impl Into<usize> for $name {
            fn into(self) -> usize {
                self.0 as _
            }
        }
    };
}

num_len!(U8Len(u8));
num_len!(U16Len(u16));
num_len!(U32Len(u32));
num_len!(U64Len(u64));

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HungryVec<T, const N: usize>(Vec<T, N>);

impl<T, const N: usize> ops::Deref for HungryVec<T, N> {
    type Target = Vec<T, N>;
    fn deref(&self) -> &Vec<T, N> {
        &self.0
    }
}

impl<T, const N: usize> ops::DerefMut for HungryVec<T, N> {
    fn deref_mut(&mut self) -> &mut Vec<T, N> {
        &mut self.0
    }
}

impl<T: InnerData, const N: usize> InnerData for HungryVec<T, N> {
    const MAX_SIZE: Option<usize> = Some(N);
    const MIN_SIZE: usize = 0;

    fn byte_size(&self) -> usize {
        self.iter().map(InnerData::byte_size).sum::<usize>()
    }

    fn read<S: ReadStream>(stream: &mut S, mut max_size: usize) -> Self {
        let mut size_remaning = max_size;

        let mut vec = Vec::new();

        Self(vec)
    }

    fn write<S: WriteStream>(&self, stream: &mut S) {
        U::from(self.len()).write(stream);
        for x in &**self {
            x.write(stream);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SizeVec<U, T, const N: usize> {
    vec: Vec<T, N>,
    _phantom: PhantomData<U>,
}

impl<U, T, const N: usize> ops::Deref for SizeVec<U, T, N> {
    type Target = Vec<T, N>;
    fn deref(&self) -> &Vec<T, N> {
        &self.vec
    }
}

impl<U, T, const N: usize> ops::DerefMut for SizeVec<U, T, N> {
    fn deref_mut(&mut self) -> &mut Vec<T, N> {
        &mut self.vec
    }
}

impl<U, T, const N: usize> InnerData for SizeVec<U, T, N>
where
    U: InnerData + Into<usize> + From<usize>,
    T: InnerData,
{
    const MAX_SIZE: Option<usize> = match (T::MAX_SIZE, U::MAX_SIZE) {
        (Some(max_size), Some(len_max_size)) if (N < 64) => Some(len_max_size + N * max_size),
        _ => None,
    };
    const MIN_SIZE: usize = U::MIN_SIZE;

    fn byte_size(&self) -> usize {
        U::from(self.len()).byte_size() + self.iter().map(InnerData::byte_size).sum::<usize>()
    }

    fn read<S: ReadStream>(stream: &mut S, mut max_size: usize) -> Self {
        let u_len = U::read(stream, max_size);
        max_size -= u_len.byte_size();

        let len: usize = u_len.into();
        let mut field_size = T::MAX_SIZE
            .map_or(max_size, |c_max_size| (len * c_max_size).min(max_size))
            - len * T::MIN_SIZE;

        Self {
            vec: (0..len)
                .map(|_| {
                    field_size += T::MIN_SIZE;
                    let value = T::read(stream, field_size);
                    field_size -= value.byte_size();
                    value
                })
                .collect(),
            _phantom: PhantomData,
        }
    }

    fn write<S: WriteStream>(&self, stream: &mut S) {
        U::from(self.len()).write(stream);
        for x in &**self {
            x.write(stream);
        }
    }
}

impl<const N: usize> InnerData for String<N> {
    const MAX_SIZE: Option<usize> = Some(1 + N);
    const MIN_SIZE: usize = 1;

    fn byte_size(&self) -> usize {
        1 + self.len()
    }

    fn read<S: ReadStream>(stream: &mut S, max_size: usize) -> Self {
        let len = u8::read(stream, 1) as usize;
        assert!(
            len < max_size,
            "String length too long to fit in `max_size`"
        );

        let bytes = &mut [0u8; N][..len];
        stream.read(bytes);
        core::str::from_utf8(&bytes)
            .expect("Read a bad UTF-8 string")
            .into()
    }

    fn write<S: WriteStream>(&self, stream: &mut S) {
        (self.len() as u8).write(stream);
        stream.write(self.as_bytes());
    }
}

#[macro_export]
macro_rules! inner_data_enum {
    () => {};
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident: $ty:ident {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident = $value:literal,
            )+
        }
        $($tail:tt)*
    ) => {
        $(#[$outer])*
        #[repr($ty)]
        $vis enum $name {
            $(
                $(#[$variant_attr])*
                $variant = $value,
            )+
        }
        impl $crate::InnerData for $name {
            const MAX_SIZE: Option<usize> = <$ty as $crate::InnerData>::MAX_SIZE;
            const MIN_SIZE: usize = <$ty as $crate::InnerData>::MIN_SIZE;

            fn byte_size(&self) -> usize {
                Self::MIN_SIZE
            }
            fn write<S: WriteStream>(&self, stream: &mut S) {
                <$ty as $crate::InnerData>::write(&(*self as $ty), stream);
            }
            fn read<S: ReadStream>(stream: &mut S, max_size: usize) -> Self {
                match <$ty as $crate::InnerData>::read(stream, max_size) {
                    $($value => Self::$variant,)+
                    x => panic!("Read an the unsupported value {x} for `{}`", stringify!($name)),
                }
            }
        }
        $crate::stream::inner_data_enum!($($tail)*);
    };
}
pub use inner_data_enum;
