use std::borrow::Cow;

pub fn cow_map<'a, T, U>(
    this: Cow<'_, T>,
    borrowed_f: impl Fn(&T) -> &U,
    owned_f: impl Fn(T::Owned) -> U::Owned,
) -> Cow<'_, U>
where
    T: ?Sized + 'a,
    T: ToOwned,
    U: ?Sized + 'a,
    U: ToOwned,
{
    use Cow as E;
    match this {
        E::Borrowed(b) => E::Borrowed(borrowed_f(b)),
        E::Owned(o) => E::Owned(owned_f(o)),
    }
}

pub fn cow_into_owned<'a, T>(this: Cow<'a, T>) -> T::Owned
where
    T: ?Sized + 'a,
    T: ToOwned,
{
    use Cow as E;
    match this {
        E::Borrowed(b) => b.to_owned(),
        E::Owned(o) => o,
    }
}

pub fn cow_make_owned<'a, T>(this: Cow<'a, T>) -> Cow<'static, T>
where
    T: ?Sized + 'a,
    T: ToOwned,
{
    Cow::Owned(cow_into_owned(this))
}
