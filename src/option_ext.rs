pub trait OptionExt<T> {
    fn is_none_or<'a, F>(&'a self, f: F) -> bool where F: FnOnce(&'a T) -> bool, T: 'a;
}

impl<T> OptionExt<T> for Option<T> {
    fn is_none_or<'a, F>(&'a self, f: F) -> bool where F: FnOnce(&'a T) -> bool {
        match *self {
            Some(ref inner) => f(inner),
            None => true,
        }
    }
}