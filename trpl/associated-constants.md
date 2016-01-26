% Ассоциированные константы

С включенной возможностью `associated_consts` вы можете определить константы
вроде этой:

```rust
#![feature(associated_consts)]

trait Foo {
    const ID: i32;
}

impl Foo for i32 {
    const ID: i32 = 1;
}

fn main() {
    assert_eq!(1, i32::ID);
}
```

Любая реализация `Foo` должна будет определить `ID`. Без этого определения:

```rust,ignore
#![feature(associated_consts)]

trait Foo {
    const ID: i32;
}

impl Foo for i32 {
}
```

выдаст ошибку

```text
error: not all trait items implemented, missing: `ID` [E0046]
     impl Foo for i32 {
     }
```

Также может быть реализовано значение по умолчанию:

```rust
#![feature(associated_consts)]

trait Foo {
    const ID: i32 = 1;
}

impl Foo for i32 {
}

impl Foo for i64 {
    const ID: i32 = 5;
}

fn main() {
    assert_eq!(1, i32::ID);
    assert_eq!(5, i64::ID);
}
```

Как вы можете видеть, при реализации `Foo`, можно оставить константу
неопределенной, как в случае для `i32`. Тогда будет использовано значение по
умолчанию. Но также можно и добавить собственное определение, как в случае для
`i64`.

Ассоциированные константы могут быть ассоциированы не только с типажом. Это
также прекрасно работает и с блоком `impl` для `struct`:

```rust
#![feature(associated_consts)]

struct Foo;

impl Foo {
    pub const FOO: u32 = 3;
}
```
