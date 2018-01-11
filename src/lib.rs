#[cfg(feature = "pretty")]
extern crate console;

use std::default::Default;
use std::error::Error;
use std::fmt;

pub struct Test {
    name: String,
    assertions: Vec<Assertion>,
}

#[derive(Debug, Default)]
pub struct Asserting<'a> {
    name: Option<&'a str>,
    at: Option<Location>,
    clues: Vec<String>,
}

#[derive(Debug)]
pub struct Location {
    file: &'static str,
    line: u32,
}

pub struct Assertion {
    name: String,
    at: Option<Location>,
    value: AssertionValue,
    clues: Vec<String>,
}

enum AssertionValue {
    Passed,
    Failed,
    Errored(Box<Error>),
}
#[cfg(feature = "pretty")]
trait Styler {
    fn passed(&self) -> bool;

    fn style(&self) -> console::Style {
        if self.passed() {
            console::Style::new().green()
        } else {
            console::Style::new().red()
        }
    }

    fn check(&self) -> console::StyledObject<console::Emoji> {
        if self.passed() {
            console::style(console::Emoji("✔", "+")).green()
        } else {
            console::style(console::Emoji("✖", "x")).red()
        }
    }
}

// ===== impl Asserting =====

impl<'a> Asserting<'a> {
    pub fn at(mut self, file: &'static str, line: u32) -> Self {
        self.at = Some(Location { file, line });
        self
    }

    pub fn with_clue<T>(mut self, clue: &T, name: &str) -> Self
    where
        T: fmt::Debug,
    {
        self.clues.push(format!("{} = {:?}", name, clue));
        self
    }

    pub fn that(that: &'a str) -> Self {
        Asserting {
            name: Some(that),
            ..Default::default()
        }
    }

    pub fn is_true(self, value: bool) -> Assertion {
        let value = if value {
            AssertionValue::Passed
        } else {
            AssertionValue::Failed
        };
        Assertion {
            name: self.name.map(String::from).unwrap(),
            at: self.at,
            value,
            clues: self.clues,
        }
    }
}

// ===== impl Assertion =====

impl Assertion {
    pub fn with_clue<T>(&mut self, clue: &T, name: &str) -> &mut Self
    where
        T: fmt::Debug,
    {
        self.clues.push(format!("{} = {:?}", name, clue));
        self
    }

    pub fn passed(&self) -> bool {
        if let AssertionValue::Passed = self.value {
            true
        } else {
            false
        }
    }
}

#[cfg(feature = "pretty")]
impl Styler for Assertion {
    fn passed(&self) -> bool { self.passed() }
}

#[cfg(feature = "pretty")]
impl fmt::Display for Assertion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let style = self.style();
        write!(
            f,
            "{check} {name}\n",
            check = self.check(),
            name = style.apply_to(&self.name)
        )?;
        if !self.passed() {
            if let Some(ref loc) = self.at {
                write!(f, "  {}\n", style.apply_to(loc))?;
            }
            for ref clue in &self.clues {
                write!(f, "  {}\n", style.apply_to(clue))?;
            }
        };
        Ok(())
    }
}

// ===== impl Location =====

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "at {}:{}", self.file, self.line)
    }
}

#[macro_export]
macro_rules! assert_that {
    ($assertion:expr) => {
        $crate::Asserting::that(stringify!($assertion))
            .at(file!(), line!())
            .is_true($assertion)
    };
    ($assertion:expr, $(clue: $clue:expr),+) => {
        assert_that!($assertion)
            $( .with_clue(&$clue, stringify!($clue)) )+
    };
}

#[macro_export]
macro_rules! assert_equal {
    ($a:expr, $b:expr) => { assert_that!($a == $b, clue: $a, clue: $b) };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn basic_equals() {
        let one = 1;
        let two = 2;
        println!("{}", assert_equal!(one, two));
        println!("{}", assert_equal!(one, 1));
    }
}
