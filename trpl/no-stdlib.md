% Без stdlib

По умолчанию, `std` компонуется с каждым контейнером Rust. В некоторых случаях это
нежелательно, и этого можно избежать с помощью атрибута `#![no_std]`,
примененного (привязанного) к контейнеру.

```ignore
// a minimal library
#![crate_type="lib"]
#![feature(no_std)]
#![no_std]
# // fn main() {} tricked you, rustdoc!
```

Очевидно, должно быть нечто большее, чем просто библиотеки: `#[no_std]` можно
использовать с исполняемыми контейнерами, а управлять точкой входа можно двумя
способами: с помощью атрибута `#[start]`, или с помощью переопределения
прокладки (shim) для C функции `main` по умолчанию на вашу собственную.

В функцию, помеченную атрибутом `#[start]`, передаются параметры командной
строки в том же формате, что и в C:

```rust
#![feature(lang_items, start, no_std, libc)]
#![no_std]

// Pull in the system libc library for what crt0.o likely requires
extern crate libc;

// Entry point for this program
#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    0
}

// These functions and traits are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
# // fn main() {} tricked you, rustdoc!
```

Чтобы переопределить вставленную компилятором прокладку `main`, нужно сначала
отключить ее с помощью `#![no_main]`, а затем создать соответствующий символ с
правильным ABI и правильным именем, что также потребует переопределение
искажения (коверкания) имен компилятором (`#[no_mangle]`):

```ignore
#![feature(no_std)]
#![no_std]
#![no_main]
#![feature(lang_items, start)]

extern crate libc;

#[no_mangle] // для уверенности в том, что этот символ будет называться `main` на выходе
pub extern fn main(argc: i32, argv: *const *const u8) -> i32 {
    0
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
# // fn main() {} tricked you, rustdoc!
```


В настоящее время компилятор делает определенные предположения о символах,
которые доступны для вызова в исполняемом контейнере. Как правило, эти функции
предоставляются стандартной библиотекой, но если она не используется, то вы
должны определить их самостоятельно.

Первая из этих трех функций, `stack_exhausted`, вызывается тогда, когда
обнаруживается (происходит) переполнение стека. Эта функция имеет ряд
ограничений, касающихся того, как она может быть вызвана и того, что она должна
делать, но если регистр предела стека не поддерживается, то поток всегда имеет
«бесконечный стек» и эта функция не должна быть вызвана (получить управление,
срабатывать).

Вторая из этих трех функций, `eh_personality`, используется в механизме
обработки ошибок компилятора. Она часто отображается на функцию personality
(специализации) GCC (для получения дополнительной информации смотри [реализацию
libstd](http://doc.rust-lang.org/std/rt/unwind/index.html)), но можно с
уверенностью сказать, что для контейнеров, которые не вызывают панику, эта
функция никогда не будет вызвана. Последняя функция, `panic_fmt`, также
используются в механизме обработки ошибок компилятора.

## Использование основной библиотеки (libcore)

> **Примечание**: структура основной библиотеки (core) является нестабильной, и
> поэтому рекомендуется использовать стандартную библиотеку (std) там, где это
> возможно.

С учетом указанных выше методов, у нас есть чисто-металлический исполняемый код
работает Rust. Стандартная библиотека предоставляет немало функциональных
возможностей, однако, для Rust также важна производительность. Если стандартная
библиотека не соответствует этим требованиям, то вместо нее может быть
использована [libcore](http://doc.rust-lang.org/core/index.html).

Основная библиотека имеет очень мало зависимостей и гораздо более компактна, чем
стандартная библиотека. Кроме того, основная библиотека имеет большую часть
необходимой функциональности для написания идиоматического и эффективного кода
на Rust.

В качестве примера приведем программу, которая вычисляет скалярное произведение
двух векторов, предоставленных из кода C, и использует идиоматические практики
Rust.

```ignore
#![feature(lang_items, start, no_std, core, libc)]
#![no_std]

# extern crate libc;
extern crate core;

use core::prelude::*;

use core::mem;

#[no_mangle]
pub extern fn dot_product(a: *const u32, a_len: u32,
                          b: *const u32, b_len: u32) -> u32 {
    use core::raw::Slice;

    // Convert the provided arrays into Rust slices.
    // The core::raw module guarantees that the Slice
    // structure has the same memory layout as a &[T]
    // slice.
    //
    // This is an unsafe operation because the compiler
    // cannot tell the pointers are valid.
    let (a_slice, b_slice): (&[u32], &[u32]) = unsafe {
        mem::transmute((
            Slice { data: a, len: a_len as usize },
            Slice { data: b, len: b_len as usize },
        ))
    };

    // Iterate over the slices, collecting the result
    let mut ret = 0;
    for (i, j) in a_slice.iter().zip(b_slice.iter()) {
        ret += (*i) * (*j);
    }
    return ret;
}

#[lang = "panic_fmt"]
extern fn panic_fmt(args: &core::fmt::Arguments,
                    file: &str,
                    line: u32) -> ! {
    loop {}
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
# #[start] fn start(argc: isize, argv: *const *const u8) -> isize { 0 }
# fn main() {}
```

Обратите внимание, что здесь, в отличае от примеров, рассмотренных выше, есть
один дополнительный lang элемент `panic_fmt`. Он должен быть определён
потребителями libcore, потому что основная библиотека объявляет панику, но не
определяет её. lang элемент `panic_fmt` определяет панику для этого
контейнера, и необходимо гарантировать, что он никогда не возвращает значение.

Как видно в этом примере, основная библиотека предназначена для предоставления
всей мощи Rust при любых обстоятельствах, независимо от требований платформы.
Дополнительные библиотеки, такие как liballoc, добавляют функциональность для
libcore, для работы которой нужно сделать некоторые платформо-зависимые
предположения; но эти библиотеки всё равно более переносимы, чем стандартная
библиотека в целом.
