
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
    [] => { $crate::either_n::Never };
    [$A:ty] => { $A };
    [$A:ty, $B:ty] => { $crate::either_n::Either2<$A, $B> };
    [$A:ty, $B:ty, $C:ty] => { $crate::either_n::Either3<$A, $B, $C> };
    [$A:ty, $B:ty, $C:ty, $D:ty] => { $crate::either_n::Either4<$A, $B, $C, $D> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty] => { $crate::either_n::Either5<$A, $B, $C, $D, $E> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty] => { $crate::either_n::Either6<$A, $B, $C, $D, $E, $F> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty] => { $crate::either_n::Either7<$A, $B, $C, $D, $E, $F, $G> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty] => { $crate::either_n::Either8<$A, $B, $C, $D, $E, $F, $G, $H> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty] => { $crate::either_n::Either9<$A, $B, $C, $D, $E, $F, $G, $H, $I> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty] => { $crate::either_n::Either10<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty] => { $crate::either_n::Either11<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty] => { $crate::either_n::Either12<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty] => { $crate::either_n::Either13<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty] => { $crate::either_n::Either14<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty] => { $crate::either_n::Either15<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty] => { $crate::either_n::Either16<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty] => { $crate::either_n::Either17<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty, $R:ty] => { $crate::either_n::Either18<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q, $R> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty, $R:ty, $S:ty] => { $crate::either_n::Either19<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q, $R, $S> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty, $R:ty, $S:ty, $T:ty] => { $crate::either_n::Either20<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q, $R, $S, $T> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty, $R:ty, $S:ty, $T:ty, $U:ty] => { $crate::either_n::Either21<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q, $R, $S, $T, $U> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty, $R:ty, $S:ty, $T:ty, $U:ty, $V:ty] => { $crate::either_n::Either22<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q, $R, $S, $T, $U, $V> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty, $R:ty, $S:ty, $T:ty, $U:ty, $V:ty, $W:ty] => { $crate::either_n::Either23<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q, $R, $S, $T, $U, $V, $W> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty, $R:ty, $S:ty, $T:ty, $U:ty, $V:ty, $W:ty, $X:ty] => { $crate::either_n::Either24<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q, $R, $S, $T, $U, $V, $W, $X> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty, $R:ty, $S:ty, $T:ty, $U:ty, $V:ty, $W:ty, $X:ty, $Y:ty] => { $crate::either_n::Either25<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q, $R, $S, $T, $U, $V, $W, $X, $Y> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty, $R:ty, $S:ty, $T:ty, $U:ty, $V:ty, $W:ty, $X:ty, $Y:ty, $Z:ty] => { $crate::either_n::Either26<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q, $R, $S, $T, $U, $V, $W, $X, $Y, $Z> };
    [$A:ty, $B:ty, $C:ty, $D:ty, $E:ty, $F:ty, $G:ty, $H:ty, $I:ty, $J:ty, $K:ty, $L:ty, $M:ty, $N:ty, $O:ty, $P:ty, $Q:ty, $R:ty, $S:ty, $T:ty, $U:ty, $V:ty, $W:ty, $X:ty, $Y:ty, $Z:ty, $($Rest:ty),+] => { $crate::Either![$crate::either_n::Either26<$A, $B, $C, $D, $E, $F, $G, $H, $I, $J, $K, $L, $M, $N, $O, $P, $Q, $R, $S, $T, $U, $V, $W, $X, $Y, $Z>, $($Rest),+] };
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
impl_either!(Either8, 8, A, B, C, D, E, F, G, H);
impl_either!(Either9, 9, A, B, C, D, E, F, G, H, I);
impl_either!(Either10, 10, A, B, C, D, E, F, G, H, I, J);
impl_either!(Either11, 11, A, B, C, D, E, F, G, H, I, J, K);
impl_either!(Either12, 12, A, B, C, D, E, F, G, H, I, J, K, L);
impl_either!(Either13, 13, A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_either!(Either14, 14, A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_either!(Either15, 15, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_either!(Either16, 16, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
impl_either!(Either17, 17, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_either!(Either18, 18, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
impl_either!(Either19, 19, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
impl_either!(Either20, 20, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
impl_either!(Either21, 21, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
impl_either!(Either22, 22, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
impl_either!(Either23, 23, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
impl_either!(Either24, 24, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
impl_either!(Either25, 25, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
impl_either!(Either26, 26, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);