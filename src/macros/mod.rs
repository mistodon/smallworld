#![macro_use]

macro_rules! component
{
    ($typename:ident) =>
    {
        impl ::specs::Component for $typename
        {
            type Storage = ::specs::VecStorage<$typename>;
        }
    }
}
