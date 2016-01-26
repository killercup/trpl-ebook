% Перегрузка операций

Rust позволяет ограниченную форму перегрузки операций. Есть определенные
операции, которые могут быть перегружены. Есть специальные типажи, которые вы
можете реализовать для поддержки конкретной операции между типами. В результате
чего перегружается операция.

Например, операция `+` может быть перегружена с помощью типажа `Add`:

```rust
use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };

    let p3 = p1 + p2;

    println!("{:?}", p3);
}
```

В `main` мы можем использовать операцию `+` для двух `Point`, так как мы
реализовали типаж `Add<Output=Point>` для `Point`.

Есть целый ряд операций, которые могут быть перегружены таким образом, и все
связанные с этим типажи расположены в модуле [`std::ops`][stdops]. Проверьте эту
часть документации для получения полного списка.

[stdops]: http://doc.rust-lang.org/std/ops/index.html

Реализация этих типажей следует паттерну. Давайте посмотрим на типаж
[`Add`][add] более детально:

```rust
# mod foo {
pub trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
# }
```

[add]: http://doc.rust-lang.org/std/ops/trait.Add.html

В общей сложности здесь присутствуют три типа: тип `impl Add`, который мы
реализуем, тип `RHS`, который по умолчанию равен `Self` и тип `Output`. Для
выражения `let z = x + y`: `x` — это тип `Self`, `y` — это тип `RHS`, а `z` -
это тип `Self::Output`.

```rust
# struct Point;
# use std::ops::Add;
impl Add<i32> for Point {
    type Output = f64;

    fn add(self, rhs: i32) -> f64 {
        // add an i32 to a Point and get an f64
# 1.0
    }
}
```

позволит вам сделать следующее:

```rust,ignore
let p: Point = // ...
let x: f64 = p + 2i32;
```

# Использование типажей операций в обобщённых структурах

Теперь, когда мы знаем, как реализованы типажи операций, мы можем реализовать
наш типаж `HasArea` и структуру `Square` из [главы о типажах][traits] более
общим образом:

[traits]: traits.html

```rust
use std::ops::Mul;

trait HasArea<T> {
    fn area(&self) -> T;
}

struct Square<T> {
    x: T,
    y: T,
    side: T,
}

impl<T> HasArea<T> for Square<T>
        where T: Mul<Output=T> + Copy {
    fn area(&self) -> T {
        self.side * self.side
    }
}

fn main() {
    let s = Square {
        x: 0.0f64,
        y: 0.0f64,
        side: 12.0f64,
    };

    println!("Площадь s: {}", s.area());
}
```

Мы просто объявляем тип-параметр `T` и используем его вместо `f64` в определении
`HasArea` и `Square`. В реализации нужно сделать более хитрые изменения:

```ignore
impl<T> HasArea<T> for Square<T>
        where T: Mul<Output=T> + Copy { ... }
```

Чтобы реализовать `area`, мы должны мочь умножить операнды друг на друга,
поэтому мы объявляем `T` как реализующий `std::ops::Mul`. Как и `Add`, `Mul`
принимает параметр `Output`: т.к. мы знаем, что числа не меняют своего типа,
когда их умножают, `Output` также объявлен как `T`. `T` также должен
поддерживать копирование, чтобы Rust не пытался переместить `self.side` в
возвращаемое значение.
