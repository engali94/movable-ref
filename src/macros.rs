#[doc(hidden)]
#[macro_export]
macro_rules! selfref_accessors {
    (impl $owner:ty { $get:ident, $get_mut:ident : $field:ident -> $t:ty }) => {
        impl $owner {
            pub fn $get(&self) -> &$t {
                self.$field.get()
            }
            pub fn $get_mut(&mut self) -> &mut $t {
                self.$field.get_mut()
            }
        }
    };
    (impl $owner:ty { $get:ident : $field:ident -> $t:ty }) => {
        impl $owner {
            pub fn $get(&self) -> &$t {
                self.$field.get()
            }
        }
    };
}
