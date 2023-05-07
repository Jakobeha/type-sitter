
/// Stable version of the never type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Never {}

/// Arbitrary-sized either type:
///
/// - If there are 0 arguments, it is [Never]
/// - If there is 1 argument, it is that type
/// - If there are too many arguments (over 26), it will be divided into an either of eithers.
/// - Otherwise, chooses between [Either2] and [Either26]
// This is a type macro so pascal case is appropriate
#[allow(non_snake_case)]
#[macro_export]
macro_rules! Either {
    [] => { $crate::Never };
    [$A:ty] => { $A };
    [$A:ty, $B:ty] => { $crate::Either2<$A, $B> };
    [$A:ty, $B:ty, $C:ty] => { $crate::Either3<$A, $B, $C> };
    [$A:ty, $B:ty, $C:ty, $D:ty] => { $crate::Either4<$A, $B, $C, $D> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty] => { $crate::Either5<$A, $B, $C, $D, $E> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty] => { $crate::Either6<$A, $B, $C, $D, $E, $F> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty] => { $crate::Either7<$A, $B, $C, $D, $E, $F, $G> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty] => { $crate::Either8<$A, $B, $C, $D, $E, $F, $G, $H> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty] => { $crate::Either9<$A, $B, $C, $D, $E, $F, $G, $H, $I> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty] => { $crate::Either10<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty] => { $crate::Either11<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty] => { $crate::Either12<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty] => { $crate::Either13<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty] => { $crate::Either14<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty] => { $crate::Either15<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty] => { $crate::Either16<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty] => { $crate::Either17<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty, $R:ty] => { $crate::Either18<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q, $R> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty, $R:ty, $S:ty] => { $crate::Either19<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q, $R, $S> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty, $R:ty, $S:ty, $T:ty] => { $crate::Either20<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q, $R, $S, $T> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty, $R:ty, $S:ty, $T:ty, $U:ty] => { $crate::Either21<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q, $R, $S, $T, $U> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty, $R:ty, $S:ty, $T:ty, $U:ty, $V:ty] => { $crate::Either22<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q, $R, $S, $T, $U, $V> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty, $R:ty, $S:ty, $T:ty, $U:ty, $V:ty, $W:ty] => { $crate::Either23<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q, $R, $S, $T, $U, $V, $W> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty, $R:ty, $S:ty, $T:ty, $U:ty, $V:ty, $W:ty, $X:ty] => { $crate::Either24<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q, $R, $S, $T, $U, $V, $W, $X> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty, $R:ty, $S:ty, $T:ty, $U:ty, $V:ty, $W:ty, $X:ty, $Y:ty] => { $crate::Either25<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q, $R, $S, $T, $U, $V, $W, $X, $Y> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty, $R:ty, $S:ty, $T:ty, $U:ty, $V:ty, $W:ty, $X:ty, $Y:ty, $Z:ty] => { $crate::Either26<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q, $R, $S, $T, $U, $V, $W, $X, $Y, $Z> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty, $R:ty, $S:ty, $T:ty, $U:ty, $V:ty, $W:ty, $X:ty, $Y:ty, $Z:ty, $($Rest:ty),+] => { $crate::Either![$crate::Either26<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q, $R, $S, $T, $U, $V, $W, $X, $Y, $Z>, $($Rest),+] };
}

macro_rules! impl_either {
    ($name:ident, $num:literal, ($($T:ident),+), ($($t:ident),+)) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[doc = concat!("A tagged union of ", stringify!($num), " types")]
        pub enum $name<$($T),+> {
            $($T($T)),+
        }

        impl<'tree, $($T),+> $name<$($T),+> {
            $(
            #[doc = concat!("Returns `Some` if the value is `", stringify!($T), "`")]
            #[inline]
            pub fn $t(self) -> Option<$T> {
                match self {
                    Self::$T(value) => Some(value),
                    _ => None
                }
            }
            )+
        }

        impl<'tree, $($T: $crate::TypedNode<'tree>),+> TryFrom<tree_sitter::Node<'tree>> for $name<$($T),+> {
            type Error = $crate::IncorrectKind<'tree>;

            #[inline]
            fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
                $(
                if let Ok(value) = $T::try_from(node) {
                    return Ok($name::$T(value))
                }
                )+
                Err($crate::IncorrectKind {
                    node,
                    kind: <Self as $crate::TypedNode<'tree>>::KIND
                })
            }
        }

        impl<'tree, $($T: $crate::TypedNode<'tree>),+> $crate::TypedNode<'tree> for $name<$($T),+> {
            const KIND: &'static str = concat!("{union of ", stringify!($num), "}");
            /* const KIND: &'static str = {
                $( const $T: &'static str = $T::KIND; )+
                const_format::concatcp!(" | ", $($T, " | "),+)
            }; */

            #[inline]
            fn node(&self) -> &tree_sitter::Node<'tree> {
                match self {
                    $($name::$T(value) => value.node()),+
                }
            }
        }
    };
}

impl_either!(Either2, 2, (A, B), (a, b));
impl_either!(Either3, 3, (A, B, C), (a, b, c));
impl_either!(Either4, 4, (A, B, C, D), (a, b, c, d));
impl_either!(Either5, 5, (A, B, C, D, E), (a, b, c, d, e));
impl_either!(Either6, 6, (A, B, C, D, E, F), (a, b, c, d, e, f));
impl_either!(Either7, 7, (A, B, C, D, E, F, G), (a, b, c, d, e, f, g));
impl_either!(Either8, 8, (A, B, C, D, E, F, G, H), (a, b, c, d, e, f, g, h));
impl_either!(Either9, 9, (A, B, C, D, E, F, G, H, I), (a, b, c, d, e, f, g, h, i));
impl_either!(Either10, 10, (A, B, C, D, E, F, G, H, I, J), (a, b, c, d, e, f, g, h, i, j));
impl_either!(Either11, 11, (A, B, C, D, E, F, G, H, I, J, K), (a, b, c, d, e, f, g, h, i, j, k));
impl_either!(Either12, 12, (A, B, C, D, E, F, G, H, I, J, K, L), (a, b, c, d, e, f, g, h, i, j, k, l));
impl_either!(Either13, 13, (A, B, C, D, E, F, G, H, I, J, K, L, M), (a, b, c, d, e, f, g, h, i, j, k, l, m));
impl_either!(Either14, 14, (A, B, C, D, E, F, G, H, I, J, K, L, M, N), (a, b, c, d, e, f, g, h, i, j, k, l, m, n));
impl_either!(Either15, 15, (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O), (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o));
impl_either!(Either16, 16, (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P), (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p));
impl_either!(Either17, 17, (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q), (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q));
impl_either!(Either18, 18, (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R), (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r));
impl_either!(Either19, 19, (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S), (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s));
impl_either!(Either20, 20, (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T), (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t));
impl_either!(Either21, 21, (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U), (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u));
impl_either!(Either22, 22, (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V), (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v));
impl_either!(Either23, 23, (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W), (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w));
impl_either!(Either24, 24, (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X), (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x));
impl_either!(Either25, 25, (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y), (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y));
impl_either!(Either26, 26, (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z), (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z));
