/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod whatever {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Default, Copy)]
        pub struct _bindgen_ty_1 {
            pub foo: ::std::os::raw::c_int,
        }
        #[test]
        fn bindgen_test_layout__bindgen_ty_1() {
            assert_eq!(::std::mem::size_of::<_bindgen_ty_1>() , 4usize ,
                       concat ! ( "Size of: " , stringify ! ( _bindgen_ty_1 )
                       ));
            assert_eq! (::std::mem::align_of::<_bindgen_ty_1>() , 4usize ,
                        concat ! (
                        "Alignment of " , stringify ! ( _bindgen_ty_1 ) ));
            assert_eq! (unsafe {
                        & ( * ( 0 as * const _bindgen_ty_1 ) ) . foo as *
                        const _ as usize } , 0usize , concat ! (
                        "Alignment of field: " , stringify ! ( _bindgen_ty_1 )
                        , "::" , stringify ! ( foo ) ));
        }
        impl Clone for _bindgen_ty_1 {
            fn clone(&self) -> Self { *self }
        }
        pub type typedef_struct = root::whatever::_bindgen_ty_1;
        pub const whatever_BAR: root::whatever::_bindgen_ty_2 =
            _bindgen_ty_2::BAR;
        #[repr(u32)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum _bindgen_ty_2 { BAR = 1, }
        pub use self::super::super::root::whatever::_bindgen_ty_2 as
                typedef_enum;
    }
    pub mod _bindgen_mod_id_12 {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Default, Copy)]
        pub struct _bindgen_ty_1 {
            pub foo: ::std::os::raw::c_int,
        }
        #[test]
        fn bindgen_test_layout__bindgen_ty_1() {
            assert_eq!(::std::mem::size_of::<_bindgen_ty_1>() , 4usize ,
                       concat ! ( "Size of: " , stringify ! ( _bindgen_ty_1 )
                       ));
            assert_eq! (::std::mem::align_of::<_bindgen_ty_1>() , 4usize ,
                        concat ! (
                        "Alignment of " , stringify ! ( _bindgen_ty_1 ) ));
            assert_eq! (unsafe {
                        & ( * ( 0 as * const _bindgen_ty_1 ) ) . foo as *
                        const _ as usize } , 0usize , concat ! (
                        "Alignment of field: " , stringify ! ( _bindgen_ty_1 )
                        , "::" , stringify ! ( foo ) ));
        }
        impl Clone for _bindgen_ty_1 {
            fn clone(&self) -> Self { *self }
        }
        pub type typedef_struct = root::_bindgen_mod_id_12::_bindgen_ty_1;
        pub const _bindgen_mod_id_12_BAR:
                  root::_bindgen_mod_id_12::_bindgen_ty_2 =
            _bindgen_ty_2::BAR;
        #[repr(u32)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum _bindgen_ty_2 { BAR = 1, }
        pub use self::super::super::root::_bindgen_mod_id_12::_bindgen_ty_2 as
                typedef_enum;
    }
}
