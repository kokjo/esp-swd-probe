pub mod ap;
pub mod dp;

macro_rules! make_register {
    ($name:ident, {$( $field:tt ),*} ) => {
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub u32);

        impl From<u32> for $name {
            fn from(value: u32) -> Self {
                Self(value)
            }
        }

        impl From<$name> for u32 {
            fn from(value: $name) -> u32 {
                value.0
            }
        }

        impl $name {
            $( crate::registers::impl_field_get!($field); )*
            $( crate::registers::impl_field_set!($field); )*
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut f = f.debug_struct(stringify!($name));
                let f = f.field("raw", &self.0);
                $( let f = crate::registers::impl_field_debug!(self, f, $field); )*
                f.finish()
            }
        }
    }
}

pub(crate) use make_register;

macro_rules! impl_field_get {
    (($name:ident, $start:expr, $size:expr, bool)) => {
        pub fn $name(&self) -> bool {
            ((self.0 >> $start) & (0xffffffff >> (32 - $size))) == 1
        }
    };
    (($name:ident, $start:expr, $size:expr, u8)) => {
        pub fn $name(&self) -> u8 {
            ((self.0 >> $start) & (0xffffffff >> (32 - $size))) as u8
        }
    };
    (($name:ident, $start:expr, $size:expr, $res:ident)) => {
        pub fn $name(&self) -> $res {
            ((self.0 >> $start) & (0xffffffff >> (32 - $size))).into()
        }
    };
    (($name:ident, $start:expr, $size:expr)) => {
        crate::registers::impl_field_get!(($name, $start, $size, u32));
    };
}

pub(crate) use impl_field_get;

macro_rules! impl_field_set {
    (($name:ident, $start:expr, $size:expr, $res:ident)) => {
        paste::paste! {
            pub fn [<set_ $name>](self, value: $res) -> Self {
                Self((self.0 & !((0xffffffff >> (32 - $size)) << $start)) | (Into::<u32>::into(value) << $start))
            }
        }
    };
    (($name:ident, $start:expr, $size:expr)) => {
        crate::registers::impl_field_set!(($name, $start, $size, u32));
    };
}

pub(crate) use impl_field_set;

macro_rules! impl_field_debug {
    ($self:ident, $f:ident, ($name:ident, $start:expr, $size:expr)) => {
        crate::registers::impl_field_debug!($self, $f, $name)
    };
    ($self:ident, $f:ident, ($name:ident, $start:expr, $size:expr, $res:ident)) => {
        crate::registers::impl_field_debug!($self, $f, $name)
    };
    ($self:ident, $f:ident, $name:ident) => {
        $f.field(concat!(stringify!($name), "()"), &$self.$name())
    };
}

pub(crate) use impl_field_debug;
