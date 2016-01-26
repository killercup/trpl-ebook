% Типажи

Типаж --- это возможность объяснить компилятору, что данный тип должен
предоставлять определённую функциональность.

Вы помните ключевое слово `impl`, используемое для вызова функции через
синтаксис метода?

```rust
# #![feature(core)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}
```

Типажи схожи, за исключением того, что мы определяем типаж, содержащий лишь
сигнатуру метода, а затем реализуем этот типаж для нужной структуры. Например,
как показано ниже:

```rust
# #![feature(core)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}
```

Как вы можете видеть, блок `trait` очень похож на блок `impl`. Различие состоит
лишь в том, что тело метода не определяется, а определяется только его
сигнатура. Когда мы реализуем типаж, мы используем `impl Trait for Item`, а не
просто `impl Item`.

Мы можем использовать типажи для ограничения обобщённых типов. Рассмотрим
похожую функцию, которая также не компилируется, и выводит ошибку:

```rust,ignore
fn print_area<T>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}
```

Rust выводит:

```text
error: type `T` does not implement any method in scope named `area`
```

Поскольку `T` может быть любого типа, мы не можем быть уверены, что он реализует
метод `area`. Но мы можем добавить «ограничение по типажу» к нашему обобщённому
типу `T`, гарантируя, что он будет соответствовать требованиям:

```rust
# trait HasArea {
#     fn area(&self) -> f64;
# }
fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}
```

Синтаксис `<T: HasArea>` означает «любой тип, реализующий типаж `HasArea`».
Так как типажи определяют сигнатуры типов функций, мы можем быть уверены, что
любой тип, который реализует `HasArea`, будет иметь метод `.area()`.

Вот расширенный пример того, как это работает:

```rust
# #![feature(core)]
trait HasArea {
    fn area(&self) -> f64;
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct Square {
    x: f64,
    y: f64,
    side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: HasArea>(shape: T) {
    println!("Площадь этой фигуры равна {}", shape.area());
}

fn main() {
    let c = Circle {
        x: 0.0f64,
        y: 0.0f64,
        radius: 1.0f64,
    };

    let s = Square {
        x: 0.0f64,
        y: 0.0f64,
        side: 1.0f64,
    };

    print_area(c);
    print_area(s);
}
```

Ниже показан вывод программы:

```text
Площадь этой фигуры равна 3.141593
Площадь этой фигуры равна 1
```

Как вы можете видеть, теперь `print_area` не только является обобщённой
функцией, но и гарантирует, что будет получен корректный тип. Если же мы
передадим некорректный тип:

```rust,ignore
print_area(5);
```

Мы получим ошибку времени компиляции:

```text
error: the trait `HasArea` is not implemented for the type `_` [E0277]
```

До сих пор мы добавляли реализации типажей лишь для структур, но реализовать
типаж можно для любого типа. Технически, мы _могли бы_ реализовать `HasArea` для
`i32`:

```rust
trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for i32 {
    fn area(&self) -> f64 {
        println!("это нелепо");

        *self as f64
    }
}

5.area();
```

Хотя технически это возможно, реализация методов для примитивных типов считается
плохим стилем программирования.

Может показаться, что такой подход легко приводит к бардаку в коде, однако
есть два ограничения, связанные с реализацией типажей, которые мешают коду выйти
из-под контроля. Во-первых, если типаж не определён в нашей области видимости,
он не применяется. Например, стандартная библиотека предоставляет типаж
[`Write`][write], который добавляет типу `File` функциональность ввода-вывода.
По умолчанию у `File` не будет этих методов:

[write]: https://doc.rust-lang.org/stable/std/io/trait.Write.html
```rust,ignore
let mut f = std::fs::File::open("foo.txt").ok().expect("Не могу открыть foo.txt");
let buf = b"whatever"; // литерал строки байт. buf: &[u8; 8]
let result = f.write(buf);
# result.unwrap(); // игнорируем ошибку
```

Вот ошибка:

```text
error: type `std::fs::File` does not implement any method in scope named `write`
let result = f.write(buf);
               ^~~~~~~~~~
```

Сначала мы должны сделать `use` для типажа `Write`:

```rust,ignore
use std::io::Write;

let mut f = std::fs::File::open("foo.txt").ok().expect("Не могу открыть foo.txt");
let buf = b"whatever";
let result = f.write(buf);
# result.unwrap(); // игнорируем ошибку
```

Это скомпилируется без ошибки.

Благодаря такой логике работы, даже если кто-то сделает что-то страшное —
например, добавит методы `i32`, это не коснётся вас, пока вы не импортируете
типаж.

Второе ограничение реализации типажей --- это то, что или типаж, или тип, для
которого вы реализуете типаж, должен быть реализован вами. Мы могли бы
определить `HasArea` для `i32`, потому что `HasArea` — это наш код. Но если бы
мы попробовали реализовать для `i32` `ToString` — типаж, предоставляемый Rust —
мы бы не смогли сделать это, потому что ни типаж, ни тип не реализован нами.

Последнее, что нужно сказать о типажах: обобщённые функции с ограничением по
типажам используют *мономорфизацию* (*mono*: один, *morph*: форма), поэтому они
диспетчеризуются статически. Что это значит? Посмотрите главу
[Типажи-объекты][to], чтобы получить больше информации.

[to]: trait-objects.html

# Множественные ограничения по типажам

Вы уже видели, как можно ограничить обобщённый параметр типа определённым
типажом:

```rust
fn foo<T: Clone>(x: T) {
    x.clone();
}
```

Если вам нужно больше одного ограничения, вы можете использовать `+`:

```rust
use std::fmt::Debug;

fn foo<T: Clone + Debug>(x: T) {
    x.clone();
    println!("{:?}", x);
}
```

Теперь тип `T` должен реализовавать как типаж `Clone`, так и типаж `Debug`.

# Утверждение where

Написание функций с несколькими обобщёнными типами и небольшим количеством
ограничений по типажам выглядит не так уж плохо, но, с увеличением количества
зависимостей, синтаксис получается более неуклюжим:

```rust
use std::fmt::Debug;

fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
    x.clone();
    y.clone();
    println!("{:?}", y);
}
```

Имя функции находится слева, а список параметров — далеко справа. Ограничения
загромождают место.

Есть решение и для этой проблемы, и оно называется «утверждение `where`»:

```rust
use std::fmt::Debug;

fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

fn bar<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

fn main() {
    foo("Привет", "мир");
    bar("Привет", "мир");
}
```

`foo()` использует синтаксис, показанный ранее, а `bar()` использует утверждение
`where`. Все, что нам нужно сделать, это убрать ограничения при определении
типов параметров, а затем добавить `where` после списка параметров. В более
длинных списках можно использовать пробелы:

```rust
use std::fmt::Debug;

fn bar<T, K>(x: T, y: K)
    where T: Clone,
          K: Clone + Debug {

    x.clone();
    y.clone();
    println!("{:?}", y);
}
```

Такая гибкость может добавить ясности в сложных ситуациях.

На самом деле `where` не только упрощает написание, это более мощная
возможность. Например:

```rust
trait ConvertTo<Output> {
    fn convert(&self) -> Output;
}

impl ConvertTo<i64> for i32 {
    fn convert(&self) -> i64 { *self as i64 }
}

// может быть вызван с T == i32
fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
    x.convert()
}

// может быть вызван с T == i64
fn inverse<T>() -> T
        // использует ConvertTo как если бы это было «ConvertFrom<i32>»
        where i32: ConvertTo<T> {
    1i32.convert()
}
```

Этот код демонстрирует дополнительные преимущества использования утверждения
`where`: оно позволяет задавать ограничение, где с левой стороны располагается
произвольный тип (в данном случае `i32`), а не только простой параметр типа
(вроде `T`).

# Методы по умолчанию

Есть еще одна особенность типажей, о которой стоит поговорить: методы по
умолчанию. Проще всего показать это на примере:

```rust
trait Foo {
    fn is_valid(&self) -> bool;

    fn is_invalid(&self) -> bool { !self.is_valid() }
}
```

В типах, реализующих типаж `Foo`, нужно реализовать метод `is_valid()`, а
`is_invalid()` будет реализован по-умолчанию. Его поведение можно
переопределить:

```rust
# trait Foo {
#     fn is_valid(&self) -> bool;
#
#     fn is_invalid(&self) -> bool { !self.is_valid() }
# }
struct UseDefault;

impl Foo for UseDefault {
    fn is_valid(&self) -> bool {
        println!("Вызван UseDefault.is_valid.");
        true
    }
}

struct OverrideDefault;

impl Foo for OverrideDefault {
    fn is_valid(&self) -> bool {
        println!("Вызван OverrideDefault.is_valid.");
        true
    }

    fn is_invalid(&self) -> bool {
        println!("Вызван OverrideDefault.is_invalid!");
        true // эта реализация противоречит сама себе!
    }
}

let default = UseDefault;
assert!(!default.is_invalid()); // печатает «Вызван UseDefault.is_valid.»

let over = OverrideDefault;
assert!(over.is_invalid()); // печатает «Вызван OverrideDefault.is_invalid!»
```

# Наследование

Иногда чтобы реализовать один типаж, нужно реализовать типажи, от которых он
зависит:

```rust
trait Foo {
    fn foo(&self);
}

trait FooBar : Foo {
    fn foobar(&self);
}
```

Типы, реализующие `FooBar`, должны реализовывать `Foo`:

```rust
# trait Foo {
#     fn foo(&self);
# }
# trait FooBar : Foo {
#     fn foobar(&self);
# }
struct Baz;

impl Foo for Baz {
    fn foo(&self) { println!("foo"); }
}

impl FooBar for Baz {
    fn foobar(&self) { println!("foobar"); }
}
```

Если мы забудем реализовать `Foo`, компилятор скажет нам об этом:

```text
error: the trait `main::Foo` is not implemented for the type `main::Baz` [E0277]
```
