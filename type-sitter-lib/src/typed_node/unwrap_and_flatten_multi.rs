use crate::{ExtraOr, IncorrectKind, NodeResult, TypedNode};

/// Useful trait to unwrap a `NodeResult<'tree, ExtraOr<'tree, T>>`
pub trait NodeResultExtraOrExt<'tree, T> {
    /// Unwrap the value, *panicking* if it's the wrong kind
    fn unwrap2(self) -> T;
    /// Unwrap the value, *panicking* if it's the wrong kind with the given message
    fn expect2(self, message: impl AsRef<str>) -> T;
    /// Flatten into a `NodeResult`, with `Err` if it's an extra
    fn flatten(self) -> NodeResult<'tree, T> where T: TypedNode<'tree>;
}

/// Useful trait to unwrap an `Option<NodeResult<'tree, T>`
pub trait OptionNodeResultExt<T> {
    /// Unwrap the value, *panicking* if it's the wrong kind
    fn unwrap2(self) -> T;
    /// Unwrap the value, *panicking* if it's the wrong kind with the given message
    fn expect2(self, message: impl AsRef<str>) -> T;
    /// Flatten into an `Option`, with `None` if it's an incorrect kind
    fn flatten(self) -> Option<T>;
}

/// Useful trait to unwrap an `Option<NodeResult<'tree, ExtraOr<'tree, T>>>`
pub trait OptionNodeResultExtraOrExt<T> {
    /// Unwrap the value, *panicking* if it's absent or the wrong kind
    fn unwrap3(self) -> T;
    /// Unwrap the value, *panicking* if it's absent or the wrong kind with the given message
    fn expect3(self, message: impl AsRef<str>) -> T;
    /// Flatten into a `Option`, with `None` if it's an incorrect or extra kind
    fn flatten2(self) -> Option<T>;
}

impl<'tree, T> NodeResultExtraOrExt<'tree, T> for NodeResult<'tree, ExtraOr<'tree, T>> {
    #[inline]
    fn unwrap2(self) -> T {
        match self {
            Err(incorrect_kind) => panic!("unwrap2 called on Err({:?})", incorrect_kind),
            Ok(ExtraOr::Extra(node)) => panic!("unwrap2 called on Ok(Extra({:?}))", node),
            Ok(ExtraOr::Regular(value)) => value,
        }
    }

    #[inline]
    fn expect2(self, message: impl AsRef<str>) -> T {
        match self {
            Err(incorrect_kind) => panic!("expect2 called on Err({:?}): {}", incorrect_kind, message.as_ref()),
            Ok(ExtraOr::Extra(node)) => panic!("expect2 called on Ok(Extra({:?})): {}", node, message.as_ref()),
            Ok(ExtraOr::Regular(value)) => value,
        }
    }

    #[inline]
    fn flatten(self) -> NodeResult<'tree, T> where T: TypedNode<'tree> {
        match self {
            Err(incorrect_kind) => Err(incorrect_kind),
            Ok(ExtraOr::Extra(node)) => Err(IncorrectKind {
                node,
                kind: T::KIND,
            }),
            Ok(ExtraOr::Regular(value)) => Ok(value),
        }
    }
}

impl<'tree, T> OptionNodeResultExt<T> for Option<NodeResult<'tree, T>> {
    #[inline]
    fn unwrap2(self) -> T {
        match self {
            None => panic!("unwrap2 called on None"),
            Some(Err(incorrect_kind)) => panic!("unwrap2 called on Some(Err({:?}))", incorrect_kind),
            Some(Ok(value)) => value,
        }
    }

    #[inline]
    fn expect2(self, message: impl AsRef<str>) -> T {
        match self {
            None => panic!("expect2 called on None: {}", message.as_ref()),
            Some(Err(incorrect_kind)) => panic!("expect2 called on Some(Err({:?})): {}", incorrect_kind, message.as_ref()),
            Some(Ok(value)) => value,
        }
    }

    #[inline]
    fn flatten(self) -> Option<T> {
        match self {
            None | Some(Err(_)) => None,
            Some(Ok(value)) => Some(value),
        }
    }
}

impl<'tree, T> OptionNodeResultExtraOrExt<T> for Option<NodeResult<'tree, ExtraOr<'tree, T>>> {
    #[inline]
    fn unwrap3(self) -> T {
        match self {
            None => panic!("Unwrap3::unwrap3 called on None"),
            Some(Err(incorrect_kind)) => panic!("Unwrap3::unwrap3 called on Some(Err({:?}))", incorrect_kind),
            Some(Ok(ExtraOr::Extra(node))) => panic!("Unwrap3::unwrap3 called on Some(Ok(Extra({:?})))", node),
            Some(Ok(ExtraOr::Regular(value))) => value,
        }
    }

    #[inline]
    fn expect3(self, message: impl AsRef<str>) -> T {
        match self {
            None => panic!("expect3 called on None: {}", message.as_ref()),
            Some(Err(incorrect_kind)) => panic!("expect3 called on Some(Err({:?})): {}", incorrect_kind, message.as_ref()),
            Some(Ok(ExtraOr::Extra(node))) => panic!("expect3 called on Some(Ok(Extra({:?}))): {}", node, message.as_ref()),
            Some(Ok(ExtraOr::Regular(value))) => value,
        }
    }

    #[inline]
    fn flatten2(self) -> Option<T> {
        match self {
            None | Some(Err(_)) | Some(Ok(ExtraOr::Extra(_))) => None,
            Some(Ok(ExtraOr::Regular(value))) => Some(value),
        }
    }
}