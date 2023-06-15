#![feature(decl_macro)]

pub trait Entity {
    type Key;
    fn index_from_key(key: Self::Key) -> usize;
}

pub macro entity_impl($key_ty:ident, $val_ty:ident) {
    struct $key_ty {
        inner: usize,
    }

    impl ::a::Entity for $val_ty {
        type Key = $key_ty;

        fn index_from_key(key: Self::Key) -> usize {
            key.inner
        }
    }
}

// does not reproduce with this uncommented, and the macro above commented
// #[macro_export]
// macro_rules! entity_impl {
//     ($key_ty:ident, $val_ty:ident) => {
//         struct $key_ty {
//             inner: usize,
//         }

//         impl ::a::Entity for $val_ty {
//             type Key = $key_ty;

//             fn index_from_key(key: Self::Key) -> usize {
//                 key.inner
//             }
//         }
//     }
// }
