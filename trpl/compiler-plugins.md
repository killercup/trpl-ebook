% Плагины к компилятору

# Введение

`rustc`, компилятор Rust, поддерживает плагины. Плагины — это разработанные
пользователями библиотеки, которые добавляют новые возможности в компилятор: это
могут быть расширения синтаксиса, дополнительные статические проверки (lints), и
другое.

Плагин — это контейнер, собираемый в динамическую библиотеку, и имеющий
отдельную функцию для регистрации расширения в `rustc`. Другие контейнеры могут
загружать эти расширения с помощью атрибута `#![plugin(...)]`. Также смотрите
раздел [`rustc::plugin`](http://doc.rust-lang.org/rustc/plugin/index.html) с
подробным описанием механизма определения и загрузки плагина.

Передаваемые в `#![plugin(foo(... args ...))]` аргументы не обрабатываются самим
`rustc`. Они передаются плагину с помощью
[метода `args`](http://doc.rust-lang.org/rustc/plugin/registry/struct.Registry.html#method.args)
структуры `Registry`.

В подавляющем большинстве случаев плагин должен использоваться *только* через
конструкцию `#![plugin]`, а не через `extern crate`. Компоновка потянула бы
внутренние библиотеки `libsyntax` и `librustc` как зависимости для вашего
контейнера. Обычно это нежелательно, и может потребоваться только если вы
собираете ещё один, другой, плагин. Статический анализ `plugin_as_library`
проверяет выполнение этой рекомендации.

Обычная практика — помещать плагины в отдельный контейнер, не содержащий
определений макросов (`macro_rules!`) и обычного кода на Rust, предназначенного
для непосредственно конечных пользователей библиотеки.

# Расширения синтаксиса

Плагины могут по-разному расширять синтаксис Rust. Один из видов расширения
синтаксиса — это процедурные макросы. Они вызываются так же, как и
[обычные макросы](macros.html), но их раскрытие производится произвольным кодом
на Rust, который оперирует
[синтаксическими деревьями](http://doc.rust-lang.org/syntax/ast/index.html) во
время компиляции.

Давайте напишем плагин
[`roman_numerals.rs`](https://github.com/rust-lang/rust/tree/master/src/test/auxiliary/roman_numerals.rs),
который реализует целочисленные литералы с римскими цифрами.

```ignore
#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;

use syntax::codemap::Span;
use syntax::parse::token;
use syntax::ast::{TokenTree, TtToken};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;  // типаж для expr_usize
use rustc::plugin::Registry;

fn expand_rn(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
        -> Box<MacResult + 'static> {

    static NUMERALS: &'static [(&'static str, u32)] = &[
        ("M", 1000), ("CM", 900), ("D", 500), ("CD", 400),
        ("C",  100), ("XC",  90), ("L",  50), ("XL",  40),
        ("X",   10), ("IX",   9), ("V",   5), ("IV",   4),
        ("I",    1)];

    let text = match args {
        [TtToken(_, token::Ident(s, _))] => token::get_ident(s).to_string(),
        _ => {
            cx.span_err(sp, "аргумент должен быть единственным идентификатором");
            return DummyResult::any(sp);
        }
    };

    let mut text = &*text;
    let mut total = 0;
    while !text.is_empty() {
        match NUMERALS.iter().find(|&&(rn, _)| text.starts_with(rn)) {
            Some(&(rn, val)) => {
                total += val;
                text = &text[rn.len()..];
            }
            None => {
                cx.span_err(sp, "неправильное римское число");
                return DummyResult::any(sp);
            }
        }
    }

    MacEager::expr(cx.expr_u32(sp, total))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("rn", expand_rn);
}
```

Теперь мы можем использовать `rn!()` как любой другой макрос:

```ignore
#![feature(plugin)]
#![plugin(roman_numerals)]

fn main() {
    assert_eq!(rn!(MMXV), 2015);
}
```

У этого подхода есть преимущества относительно простой функции `fn(&str) ->
u32`:

* Преобразование (в общем случае, произвольной сложности) выполняется во время
  компиляции;
* Проверка правильности записи литерала также производится во время компиляции;
* Можно добавить возможность использования литерала в образцах (patterns), что
  по сути позволяет создавать литералы для любого типа данных.

В дополнение к процедурным макросам, вы можете определять новые атрибуты
[`derive`](http://doc.rust-lang.org/reference.html#derive) и другие виды
расширений. Смотрите раздел
[`Registry::register_syntax_extension`](http://doc.rust-lang.org/rustc/plugin/registry/struct.Registry.html#method.register_syntax_extension)
и документацию
[перечисления `SyntaxExtension`](http://doc.rust-lang.org/syntax/ext/base/enum.SyntaxExtension.html).
В качестве более продвинутого примера с макросами, можно ознакомиться с
макросами регулярных выражений
[`regex_macros`](https://github.com/rust-lang/regex/blob/master/regex_macros/src/lib.rs).


## Советы и хитрости

Некоторые [советы по отладке макросов](macros.html#debugging-macro-code)
применимы и в случае плагинов.

Можно использовать
[`syntax::parse`](http://doc.rust-lang.org/syntax/parse/index.html), чтобы
преобразовать деревья токенов в высокоуровневые элементы синтаксиса, вроде
выражений:

```ignore
fn expand_foo(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
        -> Box<MacResult+'static> {

    let mut parser = cx.new_parser_from_tts(args);

    let expr: P<Expr> = parser.parse_expr();
```

Можно просмотреть код
[парсера `libsyntax`](https://github.com/rust-lang/rust/blob/master/src/libsyntax/parse/parser.rs),
чтобы получить представление о работе инфраструктуры разбора.

Сохраняйте [`Span`ы](http://doc.rust-lang.org/syntax/codemap/struct.Span.html)
всего, что вы разбираете, чтобы лучше сообщать об ошибках. Вы можете обернуть
ваши структуры данных в
[`Spanned`](http://doc.rust-lang.org/syntax/codemap/struct.Spanned.html).

Вызов
[`ExtCtxt::span_fatal`](http://doc.rust-lang.org/syntax/ext/base/struct.ExtCtxt.html#method.span_fatal)
сразу прервёт компиляцию. Вместо этого, лучше вызвать
[`ExtCtxt::span_err`](http://doc.rust-lang.org/syntax/ext/base/struct.ExtCtxt.html#method.span_err)
и вернуть
[`DummyResult`](http://doc.rust-lang.org/syntax/ext/base/struct.DummyResult.html),
чтобы компилятор мог продолжить работу и обнаружить дальнейшие ошибки.

Вы можете использовать
[`span_note`](http://doc.rust-lang.org/syntax/ext/base/struct.ExtCtxt.html#method.span_note)
и
[`syntax::print::pprust::*_to_string`](http://doc.rust-lang.org/syntax/print/pprust/index.html#functions)
чтобы напечатать синтаксический фрагмент для отладки.

Пример выше создавал целочисленный литерал с помощью
[`AstBuilder::expr_usize`](http://doc.rust-lang.org/syntax/ext/build/trait.AstBuilder.html#tymethod.expr_usize).
В качестве альтернативы типажу `AstBuilder`, `libsyntax` предоставляет набор
[макросов квазицитирования](http://doc.rust-lang.org/syntax/ext/quote/index.html).
Они не документированы и совсем не отполированы. Однако, эта реализация может
стать неплохой основой для улучшенной библиотеки квазицитирования, которая
работала бы как обычный плагин.


# Плагины статических проверок

Плагины могут расширять
[инфраструктуру статических проверок Rust](http://doc.rust-lang.org/reference.html#lint-check-attributes),
предоставляя новые проверки стиля кодирования, безопасности, и т.д. Полный
пример можно найти в
[`src/test/auxiliary/lint_plugin_test.rs`](https://github.com/rust-lang/rust/blob/master/src/test/auxiliary/lint_plugin_test.rs).
Здесь мы приводим его суть:

```ignore
declare_lint!(TEST_LINT, Warn,
              "Предупреждать об элементах, названных 'lintme'");

struct Pass;

impl LintPass for Pass {
    fn get_lints(&self) -> LintArray {
        lint_array!(TEST_LINT)
    }

    fn check_item(&mut self, cx: &Context, it: &ast::Item) {
        let name = token::get_ident(it.ident);
        if name.get() == "lintme" {
            cx.span_lint(TEST_LINT, it.span, "элемент называется 'lintme'");
        }
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_lint_pass(box Pass as LintPassObject);
}
```

Тогда код вроде

```ignore
#![plugin(lint_plugin_test)]

fn lintme() { }
```

выдаст предупреждение компилятора:

```txt
foo.rs:4:1: 4:16 warning: item is named 'lintme', #[warn(test_lint)] on by default
foo.rs:4 fn lintme() { }
         ^~~~~~~~~~~~~~~
```

Плагин статического анализа состоит из следующих частей:

* один или больше вызовов `declare_lint!`, которые определяют статические
  структуры [`Lint`](http://doc.rust-lang.org/rustc/lint/struct.Lint.html);

* структура, содержащая состояние, необходимое анализатору (в данном случае, его
  нет);

* реализация типажа
  [`LintPass`](http://doc.rust-lang.org/rustc/lint/trait.LintPass.html),
  определяющая, как проверять каждый элемент синтаксиса. Один `LintPass` может
  вызывать `span_lint` для нескольких различных `Lint`, но он должен
  зарегистрировать их все через метод `get_lints`.

Проходы статического анализатора — это обходы синтаксического дерева, но они
выполняются на поздних стадиях компиляции, когда уже доступа информация о типах.
Встроенные в `rustc`
[анализы](https://github.com/rust-lang/rust/blob/master/src/librustc/lint/builtin.rs)
в основном используют ту же инфрастуктуру, что и плагины статического анализа.
Смотрите их исходный код, чтобы понять, как получать информацию о типах.

Статические проверки, определяемые плагинами, управляются обычными
[атрибутами и флагами компилятора](http://doc.rust-lang.org/reference.html#lint-check-attributes),
т.е. `#[allow(test_lint)]` или `-A test-lint`. Эти идентификаторы выводятся из
первого аргумента `declare_lint!`, с учётом соответствующих преобразований
регистра букв и пунктуации.

Вы можете выполнить команду `rustc -W help foo.rs`, чтобы увидеть весь список
статических проверок, известных `rustc`, включая те, что загружаются
из`foo.rs`.
