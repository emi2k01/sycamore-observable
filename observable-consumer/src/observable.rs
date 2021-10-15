use std::collections::HashMap;
use std::hash::Hash;

pub trait Observable {
    type Reflection;

    fn into_observable(self) -> Self::Reflection;
    fn from_observable(observable: Self::Reflection) -> Self;
}

macro_rules! impl_observable_primitive {
    ($($prim:ty),*) => {
        $(
            impl Observable for $prim {
                type Reflection = $prim;

                fn into_observable(self) -> Self::Reflection {
                    self
                }

                fn from_observable(observable: Self::Reflection) -> Self {
                    observable
                }
            }
        )*
    }
}

impl_observable_primitive!(
    char, bool, isize, i8, i16, i32, i64, i128, usize, u8, u16, u32, u64, u128, f32, f64, String
);

macro_rules! impl_observable_tuple {
    ($( ($($n:tt $name:ident)+) ),+$(,)?) => {
        $(
            impl<$($name),+> Observable for ($($name,)+)
            where
                $($name: Observable,)+
            {
                type Reflection = ($(<$name>::Reflection,)+);

                fn into_observable(self) -> Self::Reflection {
                    ($(self.$n.into_observable(),)+)
                }

                fn from_observable(observable: Self::Reflection) -> Self {
                    ($($name::from_observable(observable.$n),)+)
                }
            }
        )+
    }
}

impl_observable_tuple! {
    (0 T0),
    (0 T0 1 T1),
    (0 T0 1 T1 2 T2),
    (0 T0 1 T1 2 T2 3 T3),
    (0 T0 1 T1 2 T2 3 T3 4 T4),
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5),
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6),
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7),
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8),
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9),
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10),
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11),
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12),
}

impl<T> Observable for Box<T>
where
    T: Observable + 'static,
{
    type Reflection = <T as Observable>::Reflection;

    fn into_observable(self) -> Self::Reflection {
        T::into_observable(*self)
    }

    fn from_observable(observable: Self::Reflection) -> Self {
        Box::new(T::from_observable(observable))
    }
}

impl<T> Observable for Vec<T>
where
    T: Observable + 'static,
    <T as Observable>::Reflection: Clone,
{
    type Reflection = Vec<::sycamore::reactive::Signal<<T as Observable>::Reflection>>;

    fn into_observable(self) -> Self::Reflection {
        self.into_iter()
            .map(|v| ::sycamore::reactive::Signal::new(v.into_observable()))
            .collect()
    }

    fn from_observable(observable: Self::Reflection) -> Self {
        observable
            .into_iter()
            .map(|v| T::from_observable((*v.get()).clone()))
            .collect()
    }
}

impl<K, T> Observable for HashMap<K, T>
where
    K: Eq + Hash,
    T: Observable + 'static,
    <T as Observable>::Reflection: Clone,
{
    type Reflection = HashMap<K, ::sycamore::reactive::Signal<<T as Observable>::Reflection>>;

    fn into_observable(self) -> Self::Reflection {
        self.into_iter()
            .map(|(k, v)| (k, ::sycamore::reactive::Signal::new(v.into_observable())))
            .collect()
    }

    fn from_observable(observable: Self::Reflection) -> Self {
        observable
            .into_iter()
            .map(|(k, v)| (k, T::from_observable((*v.get()).clone())))
            .collect()
    }
}

