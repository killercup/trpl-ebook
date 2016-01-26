% Универсальный синтаксис вызова функций (universal function call syntax)

Иногда, функции могут иметь одинаковые имена. Рассмотрим этот код:

```rust
trait Foo {
    fn f(&self);
}

trait Bar {
    fn f(&self);
}

struct Baz;

impl Foo for Baz {
    fn f(&self) { println!("Baz’s impl of Foo"); }
}

impl Bar for Baz {
    fn f(&self) { println!("Baz’s impl of Bar"); }
}

let b = Baz;
```

Если мы попытаемся вызвать `b.f()`, то получим ошибку:

```text
error: multiple applicable methods in scope [E0034]
b.f();
  ^~~
note: candidate #1 is defined in an impl of the trait `main::Foo` for the type
`main::Baz`
    fn f(&self) { println!("Baz’s impl of Foo"); }
    ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
note: candidate #2 is defined in an impl of the trait `main::Bar` for the type
`main::Baz`
    fn f(&self) { println!("Baz’s impl of Bar"); }
    ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

```

Нам нужен способ указать, какой конкретно метод нужен, чтобы устранить
неоднозначность. Эта возможность называется «универсальный синтаксис вызова
функций», и выглядит это так:

```rust
# trait Foo {
#     fn f(&self);
# }
# trait Bar {
#     fn f(&self);
# }
# struct Baz;
# impl Foo for Baz {
#     fn f(&self) { println!("Baz’s impl of Foo"); }
# }
# impl Bar for Baz {
#     fn f(&self) { println!("Baz’s impl of Bar"); }
# }
# let b = Baz;
Foo::f(&b);
Bar::f(&b);
```

Давайте разберемся.

```rust,ignore
Foo::
Bar::
```

Эти части вызова задают один из двух видов типажей: `Foo` и `Bar`. Это то, что
на самом деле устраняет неоднозначность между двумя методами: Rust вызывает
метод того типажа, имя которого вы используете.

```rust,ignore
f(&b)
```

Когда мы вызываем метод, используя [синтаксис вызова метода][methodsyntax], как
например `b.f()`, Rust автоматически заимствует `b`, если `f()` принимает в
качестве аргумента `&self`. В этом же случае, Rust не будет использовать
автоматическое заимствование, и поэтому мы должны явно передать `&b`.

[methodsyntax]: method-syntax.html

# Форма с угловыми скобками

Форма UFCS, о которой мы только что говорили:

```rust,ignore
Trait::method(args);
```

Это сокращенная форма записи. Ниже представлена расширенная форма записи,
которая требуется в некоторых ситуациях:

```rust,ignore
<Type as Trait>::method(args);
```

Синтаксис `<>::` является средством предоставления подсказки типа. Тип
располагается внутри `<>`. В этом случае типом является `Type as Trait`,
указывающий, что мы хотим здесь вызвать `Trait` версию метода. Часть `as Trait`
является необязательной, если вызов не является неоднозначным. То же самое что с
угловыми скобками, отсюда и короткая форма.

Вот пример использования длинной формы записи.

```rust
trait Foo {
    fn clone(&self);
}

#[derive(Clone)]
struct Bar;

impl Foo for Bar {
    fn clone(&self) {
        println!("Making a clone of Bar");

        <Bar as Clone>::clone(self);
    }
}
```

Этот код вызывает метод `clone()` типажа `Clone`, а не типажа `Foo`.
