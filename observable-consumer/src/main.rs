use std::hash::Hash;
use std::{collections::HashMap, time::Duration};

use observable_derive::Observable;
use sycamore::reactive::Signal;

/* === DATA STRUCTURES EXAMPLE === */

#[derive(Clone)]
enum Sex {
    Male,
    Female,
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum RelationshipKind {
    Romantic { cheating: bool },
    Platonic,
    Relative,
    Friendship(bool /* best friends? */),
}

#[derive(Clone)]
struct Relationship {
    // #[observable(skip)]
    duration: Duration,
    kind: RelationshipKind,
}

#[derive(Observable, Clone)]
struct Person {
    name: String,
    age: i32,
    parents: (Box<Person>, Box<Person>),
    children: Vec<Person>,
    sex: Sex,
    relationships: HashMap<RelationshipKind, Vec<Person>>,
}

/* === DERIVED TRAIT === */
trait Observable {
    type Reflection;

    fn into_observable(self) -> Self::Reflection;
    fn from_observable(observable: Self::Reflection) -> Self;
}

/* === MANUAL IMPLEMENTATION FOR COMMON AND COMPLEX TYPES === */
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
    type Reflection = Vec<Signal<<T as Observable>::Reflection>>;

    fn into_observable(self) -> Self::Reflection {
        self.into_iter()
            .map(|v| Signal::new(v.into_observable()))
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
    type Reflection = HashMap<K, Signal<<T as Observable>::Reflection>>;

    fn into_observable(self) -> Self::Reflection {
        self.into_iter()
            .map(|(k, v)| (k, Signal::new(v.into_observable())))
            .collect()
    }

    fn from_observable(observable: Self::Reflection) -> Self {
        observable
            .into_iter()
            .map(|(k, v)| (k, T::from_observable((*v.get()).clone())))
            .collect()
    }
}

/* === EXAMPLE OF GENERATED CODE === */

// TODO 1: If enum has no data other than the tags, skip the generation of the `TObservable` and use `Self` instead
enum SexObservable {
    Male,
    Female,
}

impl Observable for Sex {
    // TODO 1:
    // type Reflection = Self
    type Reflection = SexObservable;

    fn into_observable(self) -> Self::Reflection {
        match self {
            Self::Male => Self::Reflection::Male,
            Self::Female => Self::Reflection::Female,
        }
    }

    fn from_observable(observable: Self::Reflection) -> Self {
        match observable {
            Self::Reflection::Male => Self::Male,
            Self::Reflection::Female => Self::Female,
        }
    }
}

#[derive(Clone)]
enum RelationshipKindObservable {
    Romantic {
        cheating: Signal<<bool as Observable>::Reflection>,
    },
    Platonic,
    Relative,
    Friendship(Signal<<bool as Observable>::Reflection>),
}

impl Observable for RelationshipKind {
    type Reflection = RelationshipKindObservable;

    fn into_observable(self) -> Self::Reflection {
        match self {
            Self::Romantic { cheating } => Self::Reflection::Romantic {
                cheating: Signal::new(cheating.into_observable()),
            },
            Self::Platonic => Self::Reflection::Platonic,
            Self::Relative => Self::Reflection::Relative,
            Self::Friendship(best_friends) => {
                Self::Reflection::Friendship(Signal::new(best_friends.into_observable()))
            }
        }
    }

    fn from_observable(observable: Self::Reflection) -> Self {
        match observable {
            Self::Reflection::Romantic { cheating } => Self::Romantic {
                cheating: bool::from_observable((*cheating.get()).clone()),
            },
            Self::Reflection::Platonic => Self::Platonic,
            Self::Reflection::Relative => Self::Relative,
            Self::Reflection::Friendship(best_friends) => {
                Self::Friendship(bool::from_observable((*best_friends.get()).clone()))
            }
        }
    }
}

#[derive(Clone)]
struct RelationshipObservable {
    duration: Duration,
    kind: Signal<<RelationshipKind as Observable>::Reflection>,
}

impl Observable for Relationship {
    type Reflection = RelationshipObservable;

    fn into_observable(self) -> Self::Reflection {
        Self::Reflection {
            duration: self.duration,
            kind: Signal::new(RelationshipKind::into_observable(self.kind)),
        }
    }

    fn from_observable(observable: Self::Reflection) -> Self {
        Self {
            duration: observable.duration,
            kind: RelationshipKind::from_observable((*observable.kind.get()).clone()),
        }
    }
}

#[derive(Clone)]
struct PersonObservable {
    name: Signal<<String as Observable>::Reflection>,
    age: Signal<<i32 as Observable>::Reflection>,
    parents: (
        Signal<<Box<Person> as Observable>::Reflection>,
        Signal<<Box<Person> as Observable>::Reflection>,
    ),
    children: Signal<<Vec<Person> as Observable>::Reflection>,
    sex: Signal<<Sex as Observable>::Reflection>,
    relationships: Signal<<HashMap<RelationshipKind, Vec<Person>> as Observable>::Reflection>,
}

impl Observable for Person {
    type Reflection = PersonObservable;

    fn into_observable(self) -> Self::Reflection {
        Self::Reflection {
            name: Signal::new(<String as Observable>::into_observable(self.name)),
            age: Signal::new(<i32 as Observable>::into_observable(self.age)),
            parents: (
                Signal::new(<Box<Person> as Observable>::into_observable(self.parents.0)),
                Signal::new(<Box<Person> as Observable>::into_observable(self.parents.1)),
            ),
            children: Signal::new(<Vec<Person> as Observable>::into_observable(self.children)),
            sex: Signal::new(<Sex as Observable>::into_observable(self.sex)),
            relationships: Signal::new(
                <HashMap<RelationshipKind, Vec<Person>> as Observable>::into_observable(
                    self.relationships,
                ),
            ),
        }
    }

    fn from_observable(observable: Self::Reflection) -> Self {
        // no way I'm writing this. I got tired
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
