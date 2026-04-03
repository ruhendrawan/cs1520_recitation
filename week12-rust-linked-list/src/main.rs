use std::fmt;

enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

impl<T> List<T> {
    /// Creates a new empty linked list
    fn new() -> Self {
        List::Nil
    }

    /// Appends a value to the end of the linked list
    fn append(&mut self, value: T) {
        match self {
            List::Nil => {
                *self = List::Cons(value, Box::new(List::Nil));
            }
            List::Cons(_, ref mut rest) => {
                rest.append(value);
            }
        }
    }

    /// Returns the length of the linked list
    fn length(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, rest) => 1 + rest.length(),
        }
    }
}

impl<T: fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            List::Nil => write!(f, "[]"),
            List::Cons(value, rest) => {
                write!(f, "[{}", value)?;
                self.fmt_rest(rest, f)?;
                write!(f, "]")
            }
        }
    }
}

impl<T: fmt::Display> List<T> {
    fn fmt_rest(&self, rest: &Box<List<T>>, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &**rest {
            List::Nil => Ok(()),
            List::Cons(value, next) => {
                write!(f, ", {}", value)?;
                self.fmt_rest(next, f)?;
                Ok(())
            }
        }
    }
}

fn main() {
    // This should run without any errors

    let mut list = List::<i32>::new();
    list.append(10);
    list.append(20);
    list.append(30);
    list.append(40);
    list.append(50);
    println!("{}", list);
    println!("List Lenght :: {}", list.length());
}
