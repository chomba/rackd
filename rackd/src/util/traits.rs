pub trait OptionExt<T> {
    fn err_or<E>(self, err: E) -> Result<(), E>;
}

impl<T> OptionExt<T> for Option<T> {
    fn err_or<E>(self, err: E) -> Result<(), E> {
        match self {
            Some(_) => Err(err),
            None => Ok(())
        }
    }
}

// pub trait Wrap {
//     type Wrapped<T>;
// }

// #[derive(Default, Debug)]
// pub struct OptionalFields;

// impl Wrap for OptionalFields {
//     type Wrapped<T> = Option<T>;
// }

// #[derive(Default, Debug)]
// pub struct RequiredFields;

// impl Wrap for RequiredFields {
//     type Wrapped<T> = T;
// }