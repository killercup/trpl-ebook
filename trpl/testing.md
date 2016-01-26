% Тестирование

> Program testing can be a very effective way to show the presence of bugs, but
> it is hopelessly inadequate for showing their absence.
>
> Edsger W. Dijkstra, "The Humble Programmer" (1972)

> Тестирование программы может быть очень эффективным способом показать наличие
> ошибок, но оно безнадёжно неподходяще для доказательства их отсутствия.
>
> Дейкстра, Эдсгер Вибе, «The Humble Programmer» (1972)

Давайте поговорим о том, как тестировать код на Rust. Мы не будем рассказывать о
том, какой подход к тестированию Rust кода является верным. Есть много подходов,
каждый из которых имеет свое представление о правильном написании тестов. Но все
эти подходы используют одни и те же основные инструменты, и мы покажем вам
синтаксис их использования.

# Тесты с атрибутом `test`

В самом простом случае, тест в Rust — это функция, аннотированная атрибутом
`test`. Давайте создадим новый проект Cargo, который будет называться `adder`:

```bash
$ cargo new adder
$ cd adder
```

При создании нового проекта, Cargo автоматически сгенерирует простой тест. Ниже
представлено содержимое `src/lib.rs`:

```rust
#[test]
fn it_works() {
}
```

Обратите внимание на `#[test]`. Этот атрибут указывает, что это тестовая
функция. В этом примере она не имеет тела. Но такого вида функции достаточно,
чтобы удачно выполнить тест. Запуск тестов осуществляется командой `cargo test`.

```bash
$ cargo test
   Compiling adder v0.0.1 (file:///home/you/projects/adder)
     Running target/adder-91b3e234d4ed382a

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Cargo скомпилировал и запустил наши тесты. В результате мы получили выходные
данные, поделенные на два раздела: один содержит информацию о тесте, который мы
написали, а другой — информацию о тестах из документации. Но об этом позже. А
сейчас посмотрим на эту строку:

```text
test it_works ... ok
```

Обратите внимание на `it_works`. Это название нашей функции:

```rust
fn it_works() {
# }
```

Мы также получили итоговую строку:

```text
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

Так почему же наш ничего не делающий тест был выполнен удачно? Любой тест,
который не вызывает `panic!`, выполняется удачно, а любой тест, который вызывает
`panic!`, выполняется неудачно. Давайте сделаем тест, который выполнится
неудачно:

```rust
#[test]
fn it_works() {
    assert!(false);
}
```

`assert!` — это макрос, определенный в Rust, и принимающий один аргумент: если
аргумент имеет значение `true`, то ничего не происходит; если аргумент является
`false`, то вызывается `panic!`. Давайте запустим наши тесты снова:

```bash
$ cargo test
   Compiling adder v0.0.1 (file:///home/you/projects/adder)
     Running target/adder-91b3e234d4ed382a

running 1 test
test it_works ... FAILED

failures:

---- it_works stdout ----
        thread 'it_works' panicked at 'assertion failed: false', /home/steve/tmp/adder/src/lib.rs:3



failures:
    it_works

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured

thread '<main>' panicked at 'Some tests failed', /home/steve/src/rust/src/libtest/lib.rs:247
```

Rust сообщает, что наш тест выполнен неудачно:

```text
test it_works ... FAILED
```

Это же отражается в итоговой строке:

```text
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured
```

Мы также получаем ненулевой код состояния. Можно использовать `$?` на OS X и Linux:

```bash
$ echo $?
101
```

На Windows, если вы используете `cmd`:

```bash
    echo %ERRORLEVEL%
```

И если вы используете PowerShell:

```bash
    echo $LASTEXITCODE # сам код
    echo $? # логическое, успешно или не успешно
```

Это бывает полезно, если вы хотите интегрировать `cargo test` в сторонний
инструмент.

Можно инвертировать ожидаемый результат теста с помощью атрибута:
`should_panic`:

```rust
#[test]
#[should_panic]
fn it_works() {
    assert!(false);
}
```

Теперь этот тест будет выполнен удачно, если вызывается `panic!`, и неудачно,
если `panic!` не вызывается. Давайте попробуем:

```bash
$ cargo test
   Compiling adder v0.0.1 (file:///home/you/projects/adder)
     Running target/adder-91b3e234d4ed382a

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Rust предоставляет и другой макрос, `assert_eq!`, который проверяет равенство
двух аргументов:

```rust
#[test]
#[should_panic]
fn it_works() {
    assert_eq!("Hello", "world");
}
```

А теперь этот тест будет выполнен удачно или неудачно? Из-за атрибута
`should_panic` он завершится удачно:

```bash
$ cargo test
   Compiling adder v0.0.1 (file:///home/you/projects/adder)
     Running target/adder-91b3e234d4ed382a

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Тесты `should_panic` могут быть хрупкими, поскольку трудно гарантировать, что
тест не вызовет панику по неожиданной причине. Чтобы помочь в этом аспекте, к
атрибуту `should_panic` может быть добавлен необязательный параметр `expected`.
Тогда тест также будет проверять, что сообщение об ошибке содержит ожидаемый
текст. Ниже представлен более безопасный вариант приведенного выше примера:

```rust
#[test]
#[should_panic(expected = "assertion failed")]
fn it_works() {
    assert_eq!("Hello", "world");
}
```

Вот и все, что касается основ! Давайте напишем один «настоящий» тест:

```rust,ignore
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[test]
fn it_works() {
    assert_eq!(4, add_two(2));
}
```

Это распространенное использование макроса `assert_eq!`: вызывать некоторую
функцию с известными аргументами и сравнить результат её вызова с ожидаемым
результатом.

# Тесты с атрибутом `ignore`

Некоторые тесты могу занимать много времени на выполнение. Такие тесты могут
быть отключены по умолчанию с помощью атрибута `ignore`:

```rust
fn it_works() {
    assert_eq!(4, add_two(2));
}

#[test]
#[ignore]
fn expensive_test() {
    // код, который занимает час на выполнение
}
```

Теперь запустим наши тесты и видим, что `it_works` запускается, а
`expensive_test` нет:

```bash
$ cargo test
   Compiling adder v0.0.1 (file:///home/you/projects/adder)
     Running target/adder-91b3e234d4ed382a

running 2 tests
test expensive_test ... ignored
test it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Дорогостоящие тесты могут быть запущены с помощью команды
`cargo test -- --ignored`:

```bash
$ cargo test -- --ignored
     Running target/adder-91b3e234d4ed382a

running 1 test
test expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Аргумент `--ignored` — это аргумент для тестового исполняемого файла, а не для
Cargo, именно поэтому команда выглядит так `cargo test -- --ignored`.

# Тесты в модуле `test`

Есть один нюанс, из-за которого наш пример нельзя назвать идиоматичным:
отсутствует модуль тестирования. Идиоматичный вариант нашего примера будет
выглядеть примерно так:

```rust,ignore
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod test {
    use super::add_two;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
}
```

Здесь есть несколько изменений. Первое — это введение `mod test` с атрибутом
`cfg`. Модуль позволяет сгруппировать все наши тесты вместе, а также, если
нужно, определить вспомогательные функции, которые будут отделены от остальной
части контейнера. Атрибут `cfg` указывает на то, что тест будет скомпилирован,
только когда мы попытаемся запустить тесты. Это может сэкономить время
компиляции, а также гарантирует, что наши тесты полностью исключены из обычной
сборки.

Второе изменение заключается в объявлении `use`. Так как мы находимся во
внутреннем модуле, то мы должны объявить использование тестируемой функции в его
области видимости. Это может раздражать, если у вас большой модуль, и поэтому
обычно используют возможность `glob`. Давайте изменим `src/lib.rs`
соответствующим образом:

```rust,ignore

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
}
```

Обратите внимание на различие в строке с `use`. Теперь запустим наши тесты:

```bash
$ cargo test
    Updating registry `https://github.com/rust-lang/crates.io-index`
   Compiling adder v0.0.1 (file:///home/you/projects/adder)
     Running target/adder-91b3e234d4ed382a

running 1 test
test test::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Работает!

Данный подход представляет собой использование модуля `test`, содержащего
модульные тесты (unit tests). Любой код, задачей которого является только лишь
тестирование небольшого кусочка функциональности, имеет смысл перенести в этот
модуль. Но что если мы хотим написать «интеграционные тесты» (integration
tests)? Для этого следует использовать директорию `tests`.

# Тесты в директории `tests`

Чтобы написать интеграционный тест, давайте создадим директорию `tests`, и
положим в нее файл `tests/lib.rs` со следующим содержимым:

```rust,ignore
extern crate adder;

#[test]
fn it_works() {
    assert_eq!(4, adder::add_two(2));
}
```

Выглядит примерно так же, как и наши предыдущие тесты, но есть некоторые
отличия. Теперь сверху у нас `extern crate adder`. Это потому, что тесты в
директории `tests` — это отдельный контейнер, и, следовательно, мы должны
компоноваться с нашей библиотекой. Это также объясняет, почему директория
`tests` — наиболее подходящее место для написания интеграционных тестов: они
используют библиотеку, как это делал бы любой другой потребитель.

Давайте запустим их:

```bash
$ cargo test
   Compiling adder v0.0.1 (file:///home/you/projects/adder)
     Running target/adder-91b3e234d4ed382a

running 1 test
test test::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/lib-c18e7d3494509e74

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Теперь у нас появилось три раздела: запускается старый модульный тест, а также
новый интеграционный тест.

Это все, что касается директории `tests`. Модуль `test` здесь не нужен, так как
здесь всё относится к тестам.

Давайте, наконец, перейдем к третьей части: тесты в документации.

# Тесты в документации

Нет ничего лучше, чем документация с примерами. Нет ничего хуже, чем примеры,
которые на самом деле не работают, потому что код изменился с тех пор, как была
написана документация. Для того, чтобы такой ситуации не возникало, Rust
поддерживает автоматический запуск примеров в документации (имейте ввиду, что
это работает только с библиотеками). Вот дополненный `src/lib.rs` с примерами:

```rust,ignore
//! Контейнер `adder` предоставляет функции сложения чисел.
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, adder::add_two(2));
//! ```

/// Эта функция прибавляет 2 к своему аргументу.
///
/// # Examples
///
/// ```
/// use adder::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
}
```

Обратите внимание на документацию уровня модуля, начинающуюся с `//!` и на
документацию уровня функции, начинающуюся с `///`. Документация Rust
поддерживает Markdown в комментариях, поэтому блоки кода помечают тройными
символами \`. В комментарии документации обычно включают раздел `# Examples`,
содержащий примеры, такие как этот. (Примечание переводчика: заголовок `#
Examples` имеет особое значение: его нельзя написать по-другому или написать на
русском языке, иначе Rust не найдёт примеров кода в документации.)

Давайте запустим тесты снова:

```bash
$ cargo test
   Compiling adder v0.0.1 (file:///home/steve/tmp/adder)
     Running target/adder-91b3e234d4ed382a

running 1 test
test test::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/lib-c18e7d3494509e74

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 2 tests
test add_two_0 ... ok
test _0 ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

Теперь у нас запускаются все три вида тестов! Обратите внимание на имена тестов
из документации: `_0` генерируется для модульных тестов, и `add_two_0` — для
функциональных тестов. Цифры на конце будут увеличиваться автоматически, если вы
добавите еще примеров. Например, при добавлении ещё одного функционального
теста, он получит имя `add_two_1`.

Мы не рассмотрели все детали написания тестов в документации. Подробнее
смотрите главу [Документация](documentation.html).

Последнее замечание: тесты в документации *не работают* для исполняемых файлов.
Подробнее об организации файлов можно узнать в главе [Контейнеры и модули](crates-and-modules.html).
