use std::hash::Hash;
use std::{collections::HashMap, time::Duration};

use observable_derive::Observable;
use sycamore::reactive::Signal;

mod observable;
use observable::Observable;

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
struct UnitStruct;

#[derive(Observable, Clone)]
struct TupleLike(i32, String);

#[derive(Observable, Clone)]
struct Person {
    name: String,
    age: i32,
    parents: (Box<Person>, Box<Person>),
    children: Vec<Person>,
    sex: Sex,
    relationships: HashMap<RelationshipKind, Vec<Person>>,
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
    let x = Box::new(10);
    let x_observable: <Box<i32>>::Reflection = todo!();
    println!("Hello, world!");
}
