#![forbid(unsafe_code, missing_docs, future_incompatible)]
//! Macros to extract value from enums with minimum boilerplate.

#[macro_export]
/// If the given expression matches any of the given patterns, return the bound value,
/// wrapped in an [Option].
///
/// Like in a `match` expression, the pattern can be optionally followed by `if`
/// and a guard expression that has access to names bound by the pattern.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate extract;
/// let foo = Some(9);
/// assert_eq!(try_extract!(foo, Some(bar) => bar), Some(9));
///
/// let bar = Err('a');
/// assert_eq!(try_extract!(bar, Ok(qlu) | Err(qlu) if ('a'..'c').contains(&qlu) => qlu), Some('a'));
///
/// let qlu: Result<_, i32> = Ok(1);
/// assert!(try_extract!(qlu, Err(e) => e).is_none());
/// ```
macro_rules! try_extract {
    ($expression:expr, $( $pattern:pat )|+ $( if $guard:expr )? => $extract:expr $(,)?) => {
        match $expression {
            $( $pattern )|+ $( if $guard )? => Some($extract),
            _ => None,
        }
    }
}

#[macro_export]
/// Matches the given expression to any of the given patterns and returns the bound value.
/// Panics with [`unreachable`] otherwise.
///
/// Like in a `match` expression, the pattern can be optionally followed by `if`
/// and a guard expression that has access to names bound by the pattern.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate extract;
/// let foo = Some(9);
/// assert_eq!(extract!(foo, Some(bar) => bar), 9);
///
/// let bar = Err('a');
/// assert_eq!(extract!(bar, Ok(qlu) | Err(qlu) if ('a'..'c').contains(&qlu) => qlu), 'a');
/// ```
///
/// ```should_panic
/// # #[macro_use] extern crate extract;
/// let qlu = extract!(Ok(1), Err(e) => e);
/// ```
macro_rules! extract {
    ($expression:expr, $( $pattern:pat )|+ $( if $guard:expr )? => $extract:expr) => {
        $crate::extract!($expression, $( $pattern )|+ $( if $guard )? => $extract ,)
    };
    ($expression:expr, $( $pattern:pat )|+ $( if $guard:expr )? => $extract:expr , $( $err_msg:tt)*) => {
        if let Some(thing) = $crate::try_extract!($expression, $( $pattern )|+ $( if $guard )? => $extract) {
            thing
        } else {
            unreachable!($( $err_msg )*)
        }
    }
}
