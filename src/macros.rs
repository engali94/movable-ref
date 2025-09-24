#[doc(hidden)]
#[macro_export]
macro_rules! selfref_accessors {
    (impl $owner:ty { $get:ident, $get_mut:ident : $field:ident -> $t:ty }) => {
        impl $owner {
            pub fn $get(&mut self) -> &$t {
                let base = self as *const _ as *const u8;
                unsafe { self.$field.get_ref_from_base_unchecked(base) }
            }
            pub fn $get_mut(&mut self) -> &mut $t {
                let base = self as *mut _ as *mut u8;
                unsafe { self.$field.get_mut_from_base_unchecked(base) }
            }
        }
    };
    (impl $owner:ty { $get:ident : $field:ident -> $t:ty }) => {
        impl $owner {
            pub fn $get(&mut self) -> &$t {
                let base = self as *const _ as *const u8;
                unsafe { self.$field.get_ref_from_base_unchecked(base) }
            }
        }
    };
}
