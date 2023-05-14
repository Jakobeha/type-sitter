/// Re-export [once_cell::race::OnceBox], needed for lazy typed query initialization
pub type TypedQueryOnceBox<T> = once_cell::race::OnceBox<T>;