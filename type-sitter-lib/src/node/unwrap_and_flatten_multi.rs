use crate::NodeResult;

/// Useful trait to unwrap an `Option<NodeResult<'tree, T>>`
pub trait OptionNodeResultExt<T> {
    /// Unwrap the `Option` and then `NodeResult`, **panic** if either is `None`/`Err`.
    fn unwrap2(self) -> T;
    /// Unwrap the `Option` and then `NodeResult`, **panic** with the message if either is
    /// `None`/`Err`.
    fn expect2(self, message: impl AsRef<str>) -> T;
    /// Flatten into an `Option`, with `None` if the node is present but an incorrect kind.
    fn flatten(self) -> Option<T>;
}

impl<'tree, T> OptionNodeResultExt<T> for Option<NodeResult<'tree, T>> {
    #[inline]
    fn unwrap2(self) -> T {
        match self {
            None => panic!("unwrap2 called on None"),
            Some(Err(incorrect_kind)) => {
                panic!("unwrap2 called on Some(Err({:?}))", incorrect_kind)
            }
            Some(Ok(value)) => value,
        }
    }

    #[inline]
    fn expect2(self, message: impl AsRef<str>) -> T {
        match self {
            None => panic!("{}", message.as_ref()),
            Some(Err(_)) => panic!("{}", message.as_ref()),
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
