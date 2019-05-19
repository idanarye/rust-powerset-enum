pub trait WithVariant<T> {
    type With;
    fn add_possibility(self) -> Self::With;
}

pub trait WithoutVariant<V> {
    type Without;
    fn remove_possibility(self) -> Result<Self::Without, V>;
}

impl<T, E, V> WithoutVariant<V> for Result<T, E>
where E: WithoutVariant<V>,
{
    type Without = Result<T, <E as WithoutVariant<V>>::Without>;
    fn remove_possibility(self) -> Result<Self::Without, V> {
        match self {
            Ok(ok) => Ok(Ok(ok)),
            Err(err) => match err.remove_possibility() {
                Ok(remaining_err) => Ok(Err(remaining_err)),
                Err(err) => Err(err),
            },
        }
    }
}

pub trait Extract {
    fn extract<V>(self) -> Result<<Self as WithoutVariant<V>>::Without, V>
    where Self: WithoutVariant<V>;
}

impl<T> Extract for T {
    fn extract<V>(self) -> Result<<Self as WithoutVariant<V>>::Without, V>
    where T: WithoutVariant<V>
    {
        self.remove_possibility()
    }
}
