# `extract`

Macros to extract value from enums with minimum boilerplate.

## Examples

```rust
use extract::*;

let foo = Some(9);
assert_eq!(try_extract!(foo, Some(bar) => bar), Some(9));

let bar = Err('a');
assert_eq!(try_extract!(bar, Ok(qlu) | Err(qlu) if ('a'..'c').contains(&qlu) => qlu), Some('a'));

let qlu: Result<_, i32> = Ok(1);
assert!(try_extract!(qlu, Err(e) => e).is_none());

let foo = Some(9);
assert_eq!(extract!(foo, Some(bar) => bar), 9);

let bar = Err('a');
assert_eq!(extract!(bar, Ok(qlu) | Err(qlu) if ('a'..'c').contains(&qlu) => qlu), 'a');

let qlu = extract!(Ok(1), Err(e) => e); // would panic
```
