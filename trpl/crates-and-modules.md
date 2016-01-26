% Контейнеры (crates) и модули (modules)

Когда проект начинает разрастаться, то хорошей практикой разработки программного
обеспечения считается: разбить его на небольшие кусочки, а затем собрать их
вместе. Также важно иметь четко определенный интерфейс, так как часть вашей
функциональности является приватной, а часть — публичной. Для облегчения такого
рода вещей Rust обладает модульной системой.

# Основные термины: контейнеры и модули

Rust имеет два различных термина, которые относятся к модульной системе:
*контейнер* и *модуль*. Контейнер — это синоним *библиотеки* или *пакета* на
других языках. Именно поэтому инструмент управления пакетами в Rust называется
Cargo: вы пересылаете ваши контейнеры другим с помощью Cargo. Контейнеры могут
производить исполняемый файл или библиотеку, в зависимости от проекта.

Каждый контейнер имеет неявный *корневой модуль*, содержащий код для этого
контейнера. В рамках этого базового модуля можно определить дерево суб-модулей.
Модули позволяют разделить ваш код внутри контейнера.

В качестве примера, давайте сделаем контейнер *phrases*, который выдает нам
различные фразы на разных языках. Чтобы не усложнять пример, мы будем
использовать два вида фраз: «greetings» и «farewells», и два языка для этих
фраз: английский и японский (日本語). Мы будем использовать следующий шаблон
модуля:

```text
                                    +-----------+
                                +---| greetings |
                                |   +-----------+
                  +---------+   |
              +---| english |---+
              |   +---------+   |   +-----------+
              |                 +---| farewells |
+---------+   |                     +-----------+
| phrases |---+
+---------+   |                     +-----------+
              |                 +---| greetings |
              |   +----------+  |   +-----------+
              +---| japanese |--+
                  +----------+  |
                                |   +-----------+
                                +---| farewells |
                                    +-----------+
```

В этом примере, `phrases` — это название нашего контейнера. Все остальное -
модули. Вы можете видеть, что они образуют дерево, в основании которого
располагается *корень* контейнера — `phrases`.

Теперь, когда у нас есть схема, давайте определим модули в коде. Для начала
создайте новый контейнер с помощью Cargo:

```bash
$ cargo new phrases
$ cd phrases
```

Если вы помните, то эта команда создает простой проект:

```bash
$ tree .
.
├── Cargo.toml
└── src
    └── lib.rs

1 directory, 2 files
```

`src/lib.rs` — корень нашего контейнера, соответствующий `phrases` в нашей
диаграмме выше.

# Объявление модулей

Для объявления каждого из наших модулей, мы используем ключевое слово `mod`.
Давайте сделаем, чтобы наш `src/lib.rs` выглядел следующим образом:

```rust
mod english {
    mod greetings {
    }

    mod farewells {
    }
}

mod japanese {
    mod greetings {
    }

    mod farewells {
    }
}
```

После ключевого слова `mod`, вы задаете имя модуля. Имена модулей следуют
соглашениям, как и другие идентификаторы Rust: `lower_snake_case`. Содержание
каждого модуля обрамляется в фигурные скобки (`{}`).

Внутри `mod` вы можете объявить суб-`mod`. Мы можем обращаться к суб-модулям с
помощью нотации (`::`). Так выглядят обращения к нашим четырем вложенным
модулям: `english::greetings`, `english::farewells`, `japanese::greetings` и
`japanese::farewells`. Так как суб-модули располагаются в пространстве имен
своих родительских модулей, то суб-модули `english::greetings` и
`japanese::greetings` не конфликтуют, несмотря на то, что они имеют одинаковые
имена, `greetings`.

Так как в этом контейнере нет функции `main()`, и называется он `lib.rs`, Cargo
соберет этот контейнер в виде библиотеки:

```bash
$ cargo build
   Compiling phrases v0.0.1 (file:///home/you/projects/phrases)
$ ls target/debug
build  deps  examples  libphrases-a7448e02a0468eaa.rlib  native
```

`libphrase-hash.rlib` — это скомпилированный контейнер. Прежде чем мы
рассмотрим, как его можно использовать из другого контейнера, давайте
разобьем его на несколько файлов.

# Контейнеры с несколькими файлами

Если бы каждый контейнер мог состоять только из одного файла, тогда этот файл
был бы очень большими. Зачастую легче разделить контейнер на несколько файлов, и
Rust поддерживает это двумя способами.

Вместо объявления модуля наподобие:

```rust,ignore
mod english {
    // contents of our module go here
}
```

Мы можем объявить наш модуль в виде:

```rust,ignore
mod english;
```

Если мы это сделаем, то Rust будет ожидать, что найдет либо файл `english.rs`,
либо файл `english/mod.rs` с содержимым нашего модуля.

Обратите внимание, что в этих файлах вам не требуется заново объявлять модуль:
это уже сделано при изначальном объявлении `mod`.

С помощью этих двух приемов мы можем разбить наш контейнер на две директории и
семь файлов:

```bash
$ tree .
.
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── english
│   │   ├── farewells.rs
│   │   ├── greetings.rs
│   │   └── mod.rs
│   ├── japanese
│   │   ├── farewells.rs
│   │   ├── greetings.rs
│   │   └── mod.rs
│   └── lib.rs
└── target
    └── debug
        ├── build
        ├── deps
        ├── examples
        ├── libphrases-a7448e02a0468eaa.rlib
        └── native
```

`src/lib.rs` — корень нашего контейнера, и выглядит он следующим образом:

```rust,ignore
mod english;
mod japanese;
```

Эти два объявления информируют Rust, что следует искать: `src/english.rs` или
`src/english/mod.rs`, `src/japanese.rs` или `src/japanese/mod.rs`, в зависимости
от нашей структуры. В данном примере мы выбрали второй вариант из-за того, что
наши модули содержат суб-модули. И `src/english/mod.rs` и `src/japanese/mod.rs`
выглядят следующим образом:

```rust,ignore
mod greetings;
mod farewells;
```

В свою очередь, эти объявления информируют Rust, что следует искать:
`src/english/greetings.rs`, `src/japanese/greetings.rs`,
`src/english/farewells.rs`, `src/japanese/farewells.rs` или
`src/english/greetings/mod.rs`, `src/japanese/greetings/mod.rs`,
`src/english/farewells/mod.rs`, `src/japanese/farewells/mod.rs`. Так как эти
суб-модули не содержат свои собственные суб-модули, то мы выбрали
`src/english/greetings.rs` и `src/japanese/farewells.rs`. Вот так!

Содержание `src/english/greetings.rs` и `src/japanese/farewells.rs` являются
пустыми на данный момент. Давайте добавим несколько функций.

Поместите следующий код в `src/english/greetings.rs`:

```rust
fn hello() -> String {
    "Hello!".to_string()
}
```

Следующий код в `src/english/farewells.rs`:

```rust
fn goodbye() -> String {
    "Goodbye.".to_string()
}
```

Следующий код в `src/japanese/greetings.rs`:

```rust
fn hello() -> String {
    "こんにちは".to_string()
}
```

Конечно, вы можете скопировать и вставить этот код с этой страницы, или просто
напечатать что-нибудь еще. Вам совершенно не обязательно знать, что на японском
языке написано «Konnichiwa», чтобы понять как работает модульная система.

Поместите следующий код в `src/japanese/farewells.rs`:

```rust
fn goodbye() -> String {
    "さようなら".to_string()
}
```

(Это «Sayonara», если вам интересно.)

Теперь у нас есть некоторая функциональность в нашем контейнере, давайте
попробуем использовать его из другого контейнера.

# Импорт внешних контейнеров

У нас есть библиотечный контейнер. Давайте создадим исполняемый контейнер,
который импортирует и использует нашу библиотеку.

Создайте файл `src/main.rs` и положите в него следующее: (при этом он не будет
компилироваться)

```rust,ignore
extern crate phrases;

fn main() {
    println!("Hello in English: {}", phrases::english::greetings::hello());
    println!("Goodbye in English: {}", phrases::english::farewells::goodbye());

    println!("Hello in Japanese: {}", phrases::japanese::greetings::hello());
    println!("Goodbye in Japanese: {}", phrases::japanese::farewells::goodbye());
}
```

Объявление `extern crate` информирует Rust о том, что для компиляции и компоновки
кода нам нужен контейнер `phrases`. После этого объявление мы можем использовать
модули контейнера `phrases`. Как мы уже упоминали ранее, вы можете использовать
два подряд идущих символа двоеточия для обращения к суб-модулям и функциям
внутри них.

Кроме того, Cargo предполагает, что `src/main.rs` — это корень бинарного, а не
библиотечного контейнера. Теперь наш пакет содержит два контейнера: `src/lib.rs`
и `src/main.rs`. Этот шаблон является довольно распространенным для исполняемых
контейнеров: основная функциональность сосредоточена в библиотечном контейнере,
а исполняемый контейнер использует эту библиотеку. Таким образом, другие
программы также могут использовать библиотечный контейнер, к тому же такой
подход обеспечивает отделение интереса (разделение функциональности).

Хотя этот код все еще не работает. Мы получаем четыре ошибки, которые выглядят
примерно так:

```bash
$ cargo build
   Compiling phrases v0.0.1 (file:///home/you/projects/phrases)
src/main.rs:4:38: 4:72 error: function `hello` is private
src/main.rs:4     println!("Hello in English: {}", phrases::english::greetings::hello());
                                                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
note: in expansion of format_args!
<std macros>:2:25: 2:58 note: expansion site
<std macros>:1:1: 2:62 note: in expansion of print!
<std macros>:3:1: 3:54 note: expansion site
<std macros>:1:1: 3:58 note: in expansion of println!
phrases/src/main.rs:4:5: 4:76 note: expansion site
```

По умолчанию все элементы в Rust являются приватными. Давайте поговорим об этом
более подробно.

# Экспорт публичных интерфейсов

Rust позволяет точно контролировать, какие элементы вашего интерфейса являются
публичными, и поэтому по умолчанию все элементы являются приватными. Чтобы
сделать элементы публичными, вы используете ключевое слово `pub`. Давайте
сначала сосредоточимся на модуле `english`, для чего сократим файл `src/main.rs`
до этого:

```rust,ignore
extern crate phrases;

fn main() {
    println!("Hello in English: {}", phrases::english::greetings::hello());
    println!("Goodbye in English: {}", phrases::english::farewells::goodbye());
}
```

В файле `src/lib.rs` в объявлении модуля `english` давайте добавим модификатор
`pub`:

```rust,ignore
pub mod english;
mod japanese;
```

В файле `src/english/mod.rs` давайте сделаем оба модуля с модификатором `pub`:

```rust,ignore
pub mod greetings;
pub mod farewells;
```

В файле `src/english/greetings.rs` давайте добавим модификатор `pub` к
объявлению нашей функции `fn`:

```rust,ignore
pub fn hello() -> String {
    "Hello!".to_string()
}
```

А также в файле `src/english/farewells.rs`:

```rust,ignore
pub fn goodbye() -> String {
    "Goodbye.".to_string()
}
```

Теперь наши контейнеры компилируются, хотя и с предупреждениями о том, что
функции в модуле `japanese` не используются:

```bash
$ cargo run
   Compiling phrases v0.0.1 (file:///home/you/projects/phrases)
src/japanese/greetings.rs:1:1: 3:2 warning: function is never used: `hello`, #[warn(dead_code)] on by default
src/japanese/greetings.rs:1 fn hello() -> String {
src/japanese/greetings.rs:2     "こんにちは".to_string()
src/japanese/greetings.rs:3 }
src/japanese/farewells.rs:1:1: 3:2 warning: function is never used: `goodbye`, #[warn(dead_code)] on by default
src/japanese/farewells.rs:1 fn goodbye() -> String {
src/japanese/farewells.rs:2     "さようなら".to_string()
src/japanese/farewells.rs:3 }
     Running `target/debug/phrases`
Hello in English: Hello!
Goodbye in English: Goodbye.
```

Теперь, когда функции являются публичными, мы можем их использовать. Отлично!
Тем не менее, написание `phrases::english::greetings::hello()` является очень
длинным и неудобным. Rust предоставляет другое ключевое слово, для импорта имен
в текущую область, чтобы для обращения можно было использовать короткие имена.
Давайте поговорим об этом ключевом слове, `use`.

# Импорт модулей с помощью `use`

Rust предоставляет ключевое слово `use`, которое позволяет импортировать имена в
нашу локальную область видимости. Давайте изменим файл `src/main.rs`, чтобы он
выглядел следующим образом:

```rust,ignore
extern crate phrases;

use phrases::english::greetings;
use phrases::english::farewells;

fn main() {
    println!("Hello in English: {}", greetings::hello());
    println!("Goodbye in English: {}", farewells::goodbye());
}
```

Две строки, начинающиеся с `use`, импортируют соответствующие модули в локальную
область видимости, поэтому мы можем обратиться к функциям по гораздо более
коротким именам. По соглашению, при импорте функции, лучшей практикой считается
импортировать модуль, а не функцию непосредственно. Другими словами, вы _могли
бы_ сделать следующее:

```rust,ignore
extern crate phrases;

use phrases::english::greetings::hello;
use phrases::english::farewells::goodbye;

fn main() {
    println!("Hello in English: {}", hello());
    println!("Goodbye in English: {}", goodbye());
}
```

Но такой подход не является идиоматическим. Он значительно чаще приводит к
конфликту имен. Для нашей короткой программы это не так важно, но, как только
программа разрастается, это становится проблемой. Если у нас возникает конфликт
имен, то Rust выдает ошибку компиляции. Например, если мы сделаем функции
`japanese` публичными, и пытаемся скомпилировать этот код:

```rust,ignore
extern crate phrases;

use phrases::english::greetings::hello;
use phrases::japanese::greetings::hello;

fn main() {
    println!("Hello in English: {}", hello());
    println!("Hello in Japanese: {}", hello());
}
```

Rust выдаст нам сообщение об ошибке во время компиляции:

```text
   Compiling phrases v0.0.1 (file:///home/you/projects/phrases)
src/main.rs:4:5: 4:40 error: a value named `hello` has already been imported in this module [E0252]
src/main.rs:4 use phrases::japanese::greetings::hello;
                  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
Could not compile `phrases`.
```

Если мы импортируем несколько имен из одного модуля, то нам совсем не
обязательно писать одно и то же много раз. Вместо этого кода:

```rust,ignore
use phrases::english::greetings;
use phrases::english::farewells;
```

Вы можете использовать сокращение:

```rust,ignore
use phrases::english::{greetings, farewells};
```

## Реэкспорт с помощью `pub use`

Вы можете использовать `use` не просто для сокращения идентификаторов. Вы также
можете использовать его внутри вашего контейнера, чтобы реэкспортировать функцию
из другого модуля. Это позволяет представить внешний интерфейс, который может не
напрямую отображать внутреннюю организацию кода.

Давайте посмотрим на примере. Измените файл `src/main.rs` следующим образом:

```rust,ignore
extern crate phrases;

use phrases::english::{greetings,farewells};
use phrases::japanese;

fn main() {
    println!("Hello in English: {}", greetings::hello());
    println!("Goodbye in English: {}", farewells::goodbye());

    println!("Hello in Japanese: {}", japanese::hello());
    println!("Goodbye in Japanese: {}", japanese::goodbye());
}
```

Затем измените файл `src/lib.rs`, чтобы сделать модуль `japanese` с публичным:

```rust,ignore
pub mod english;
pub mod japanese;
```

Далее, убедитесь, что обе функции публичные, сперва в
`src/japanese/greetings.rs`:

```rust,ignore
pub fn hello() -> String {
    "こんにちは".to_string()
}
```

А затем в `src/japanese/farewells.rs`:

```rust,ignore
pub fn goodbye() -> String {
    "さようなら".to_string()
}
```

Наконец, измените файл `src/japanese/mod.rs` вот так:

```rust,ignore
pub use self::greetings::hello;
pub use self::farewells::goodbye;

mod greetings;
mod farewells;
```

Объявление `pub use` привносит указанную функцию в эту часть области видимости
нашей модулной иерархии. Так как мы использовали `pub use` внутри нашего модуля
`japanese`, то теперь мы можем вызывать функцию `phrases::japanese::hello()` и
функцию `phrases::japanese::goodbye()`, хотя код для них расположен в
`phrases::japanese::greetings::hello()` и
`phrases::japanese::farewells::goodbye()` соответственно. Наша внутренняя
организация не определяет наш внешний интерфейс.

В этом примере мы используем `pub use` отдельно для каждой функции, которую
хотим привнести в область `japanese`. В качестве альтернативы, мы могли бы
использовать шаблонный синтаксис, чтобы включать в себя все элементы из модуля
`greetings` в текущую область: `pub use self::greetings::*`.

Что можно сказать о `self`? По умолчанию объявления `use` используют абсолютные
пути, начинающиеся с корня контейнера. `self`, напротив, формирует эти пути
относительно текущего места в иерархии. У `use` есть еще одна особая форма: вы
можете использовать `use super::`, чтобы подняться по дереву на один уровень
вверх от вашего текущего местоположения. Некоторые предпочитают думать о `self`
как о `.`, а о `super` как о `..`, что для многих командных оболочек является
представлением для текущей директории и для родительской директории
соответственно.

Вне `use`, пути относительны: `foo::bar()` ссылаться на функцию внутри `foo`
относительно того, где мы находимся. Если же используется префикс `::`, то
`::foo::bar()` будет ссылаться на другой `foo`, абсолютный путь относительно
корня контейнера.

Кроме того, обратите внимание, что мы использовали `pub use` прежде, чем
объявили наши модули с помощью `mod`. Rust требует, чтобы объявления `use` шли в
первую очередь.

Следующий код собирается и работает:

```bash
$ cargo run
   Compiling phrases v0.0.1 (file:///home/you/projects/phrases)
     Running `target/debug/phrases`
Hello in English: Hello!
Goodbye in English: Goodbye.
Hello in Japanese: こんにちは
Goodbye in Japanese: さようなら
```
