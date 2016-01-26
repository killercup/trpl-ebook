% Синтаксис методов

Функции — это хорошо, но если вы хотите вызвать несколько связных функций для
каких-либо данных, то это может быть неудобно. Рассмотрим этот код:

```rust,ignore
baz(bar(foo)));
```

Читать данную строку кода следует слева направо, поэтому мы наблюдаем такой
порядок: «baz bar foo». Но он противоположен порядку, в котором функции будут
вызываться: «foo bar baz». Было бы классно записать вызовы в том порядке, в
котором они происходят, не так ли?

```rust,ignore
foo.bar().baz();
```

К счастью, как вы уже наверно догадались, это возможно! Rust предоставляет
возможность использовать такой *синтаксис вызова метода* с помощью ключевого
слова `impl`.

# Вызов методов

Вот как это работает:

```rust
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

fn main() {
    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area());
}
```

Этот код напечатает `12.566371`.

Мы создали структуру, которая представляет собой круг. Затем мы написали блок
`impl` и определили метод `area` внутри него.

Методы принимают специальный первый параметр, `&self`. Есть три возможных
варианта: `self`, `&self` и `&mut self`. Вы можете думать об этом специальном
параметре как о `x` в `x.foo()`. Три варианта соответствуют трем возможным видам
элемента `x`: `self` — если это просто значение в стеке, `&self` — если это
ссылка и `&mut self` — если это изменяемая ссылка. Мы передаем параметр `&self`
в метод `area`, поэтому мы можем использовать его так же, как и любой другой
параметр. Так как мы знаем, что это `Circle`, мы можем получить доступ к полю
`radius` так же, как если бы это была любая другая структура.

По умолчанию следует использовать `&self`, также как следует предпочитать
заимствование владению, а неизменные ссылки изменяемым. Вот пример, включающий
все три варианта:

```rust
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn reference(&self) {
       println!("принимаем self по ссылке!");
    }

    fn mutable_reference(&mut self) {
       println!("принимаем self по изменяемой ссылке!");
    }

    fn takes_ownership(self) {
       println!("принимаем владение self!");
    }
}
```

# Цепочка вызовов методов

Итак, теперь мы знаем, как вызвать метод, например `foo.bar()`. Но что насчет
нашего первоначального примера, `foo.bar().baz()`? Это называется «цепочка
вызовов», и мы можем сделать это, вернув `self`.

```rust
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    fn grow(&self, increment: f64) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius + increment }
    }
}

fn main() {
    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area());

    let d = c.grow(2.0).area();
    println!("{}", d);
}
```

Проверьте тип возвращаемого значения:

```rust
# struct Circle;
# impl Circle {
fn grow(&self) -> Circle {
# Circle } }
```

Мы просто указываем, что возвращается `Circle`. С помощью этого метода мы можем
создать новый круг, площадь которого будет в 100 раз больше, чем у старого.

# Статические методы

Вы также можете определить методы, которые не принимают параметр `self`. Вот
шаблон программирования, который очень распространен в коде на Rust:

```rust
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }
}

fn main() {
    let c = Circle::new(0.0, 0.0, 2.0);
}
```

Этот *статический метод*, который создает новый `Circle`. Обратите внимание, что
статические методы вызываются с помощью синтаксиса: `Struct::method()`, а не
`ref.method()`.

# Шаблон «строитель» (Builder Pattern)

Давайте предположим, что нам нужно, чтобы наши пользователи могли создавать
круги и чтобы у них была возможность задавать только те свойства, которые им
нужны. В противном случае, атрибуты `x` и `y` будут `0.0`, а `radius` будет
`1.0`. Rust не поддерживает перегрузку методов, именованные аргументы или
переменное количество аргументов. Вместо этого мы используем шаблон «строитель».
Он выглядит следующим образом:

```rust
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

struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl CircleBuilder {
    fn new() -> CircleBuilder {
        CircleBuilder { x: 0.0, y: 0.0, radius: 0.0, }
    }

    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }

    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.y = coordinate;
        self
    }

    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }

    fn finalize(&self) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius }
    }
}

fn main() {
    let c = CircleBuilder::new()
                .x(1.0)
                .y(2.0)
                .radius(2.0)
                .finalize();

    println!("площадь: {}", c.area());
    println!("x: {}", c.x);
    println!("y: {}", c.y);
}
```

Всё, что мы сделали здесь — это создали ещё одну структуру, `CircleBuilder`. В
ней мы определили методы строителя. Также мы определили метод `area()` в
`Circle`. Мы также сделали еще один метод в `CircleBuilder`: `finalize()`. Этот
метод создаёт наш окончательный `Circle` из строителя. Таким образом, мы можем
использовать методы `CircleBuilder` чтобы уточнить создание `Circle`.
