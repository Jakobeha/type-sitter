/// Stable version of the never type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Never {}

#[macro_export]
macro_rules! Either {
    [] => { $crate::either_n::Never };
    [$A:ty] => { $A };
    [$A:ty, $B:ty] => { $crate::either_n::Either2<$A, $B> };
    [$A:ty, $B:ty, $C:ty] => { $crate::either_n::Either3<$A, $B, $C> };
    [$A:ty, $B:ty, $C:ty, $D:ty] => { $crate::either_n::Either4<$A, $B, $C, $D> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty] => { $crate::either_n::Either5<$A, $B, $C, $D, $E> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty] => { $crate::either_n::Either6<$A, $B, $C, $D, $E, $F> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty] => { $crate::either_n::Either7<$A, $B, $C, $D, $E, $F, $G> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, ($Hij:ty),+] => { $crate::Either![$crate::either_n::Either7<$A, $B, $C, $D, $E, $F, $G>, ($Hij),+] };
}

macro_rules! impl_either {
    ($name:ident, $num:literal, $($T:ident),+) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[doc = concat!("A tagged union of ", stringify!($num), " types")]
        pub enum $name<$($T),+> {
            $($T($T)),+
        }
    };
}

impl_either!(Either2, 2, A, B);
impl_either!(Either3, 3, A, B, C);
impl_either!(Either4, 4, A, B, C, D);
impl_either!(Either5, 5, A, B, C, D, E);
impl_either!(Either6, 6, A, B, C, D, E, F);
impl_either!(Either7, 7, A, B, C, D, E, F, G);