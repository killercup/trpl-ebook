% Продвинутое руководстве по компоновке (advanced linking)

Распространённые ситуации, в которых требовалась компоновка с кодом на Rust, уже
были рассмотрены в предыдущих главах книги. Однако для поддержки прозрачного
взаимодействия с нативными библиотеками требуется более широкая поддержка разных
вариантов компоновки.

# Аргументы компоновки (link args)

Есть только один способ тонкой настройки компоновки — атрибут `link_args`.
Этот атрибут применяется к блокам `extern`, и указывает сырые аргументы, которые
должны быть переданы компоновщику при создании артефакта. Например:

``` no_run
#![feature(link_args)]

#[link_args = "-foo -bar -baz"]
extern {}
# fn main() {}
```

Обратите внимание, что эта возможность скрыта за `feature(link_args)`, так как
это нештатный способ компоновки. В данный момент `rustc` вызывает системный
компоновщик (на большинстве систем это `gcc`, на Windows — `link.exe`),
поэтому передача аргументов командной строки имеет смысл. Но реализация не
всегда будет такой — в будущем `rustc` может напрямую использовать LLVM для
связывания с нативными библиотеками, и тогда `link_args` станет бессмысленным.
Того же эффекта можно достигнуть с пощощью передачи `rustc` аргумента `-C
link-args`.

Крайне рекомендуется *не* использовать этот атрибут, и пользоваться вместо него
более точно определённым атрибутом `#link(...)` для блоков `extern`.

# Статическое связывание

Статическое связывание — это процесс создания артефакта, который содержит все
нужные библиотеки, и потому не потребует установленных библиотек на целевой
системе. Библиотеки на Rust по умолчанию связываются статически, поэтому
приложения и библиотеки на Rust можно использовать без установки Rust повсюду.
Напротив, нативные библиотеки (например, `libc` и `libm`) обычно связываются
динамически, но это можно изменить, и сделать чтобы они также связывались
статически.

Компоновка — это процесс, который реализуется по-разному на разных платформах.
На некоторых из них статическое связывание вообще не возможно! Этот раздел
предполагает знакомство с процессом компоновки на вашей платформе.

## Linux

По умолчанию, программы на Rust для Linux компонуются с системной `libc` и ещё
некоторыми библиотеками. Давайте посмотрим на пример на 64-битной машине с
Linux, GCC и `glibc` (самой популярной `libc` на Linux):

``` text
$ cat example.rs
fn main() {}
$ rustc example.rs
$ ldd example
        linux-vdso.so.1 =>  (0x00007ffd565fd000)
        libdl.so.2 => /lib/x86_64-linux-gnu/libdl.so.2 (0x00007fa81889c000)
        libpthread.so.0 => /lib/x86_64-linux-gnu/libpthread.so.0 (0x00007fa81867e000)
        librt.so.1 => /lib/x86_64-linux-gnu/librt.so.1 (0x00007fa818475000)
        libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007fa81825f000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007fa817e9a000)
        /lib64/ld-linux-x86-64.so.2 (0x00007fa818cf9000)
        libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007fa817b93000)
```

Иногда динамическое связывание на Linux нежелательно: например, если вы хотите
использовать возможности из новых библиотек на старых системах или на целевых
системах нет таких библиотек.

Статическое связывание возможно с альтернативной `libc`, `musl`. Вы можете
скомпилировать свою версию Rust, которая будет использовать `musl`, и установить
её в отдельную директорию, с помощью инструкции, приведённой ниже:

```text
$ mkdir musldist
$ PREFIX=$(pwd)/musldist
$
$ # Build musl
$ wget http://www.musl-libc.org/releases/musl-1.1.10.tar.gz
[...]
$ tar xf musl-1.1.10.tar.gz
$ cd musl-1.1.10/
musl-1.1.10 $ ./configure --disable-shared --prefix=$PREFIX
[...]
musl-1.1.10 $ make
[...]
musl-1.1.10 $ make install
[...]
musl-1.1.10 $ cd ..
$ du -h musldist/lib/libc.a
2.2M    musldist/lib/libc.a
$
$ # Build libunwind.a
$ wget http://llvm.org/releases/3.6.1/llvm-3.6.1.src.tar.xz
$ tar xf llvm-3.6.1.src.tar.xz
$ cd llvm-3.6.1.src/projects/
llvm-3.6.1.src/projects $ svn co http://llvm.org/svn/llvm-project/libcxxabi/trunk/ libcxxabi
llvm-3.6.1.src/projects $ svn co http://llvm.org/svn/llvm-project/libunwind/trunk/ libunwind
llvm-3.6.1.src/projects $ sed -i 's#^\(include_directories\).*$#\0\n\1(../libcxxabi/include)#' libunwind/CMakeLists.txt
llvm-3.6.1.src/projects $ mkdir libunwind/build
llvm-3.6.1.src/projects $ cd libunwind/build
llvm-3.6.1.src/projects/libunwind/build $ cmake -DLLVM_PATH=../../.. -DLIBUNWIND_ENABLE_SHARED=0 ..
llvm-3.6.1.src/projects/libunwind/build $ make
llvm-3.6.1.src/projects/libunwind/build $ cp lib/libunwind.a $PREFIX/lib/
llvm-3.6.1.src/projects/libunwind/build $ cd cd ../../../../
$ du -h musldist/lib/libunwind.a
164K    musldist/lib/libunwind.a
$
$ # Build musl-enabled rust
$ git clone https://github.com/rust-lang/rust.git muslrust
$ cd muslrust
muslrust $ ./configure --target=x86_64-unknown-linux-musl --musl-root=$PREFIX --prefix=$PREFIX
muslrust $ make
muslrust $ make install
muslrust $ cd ..
$ du -h musldist/bin/rustc
12K     musldist/bin/rustc
```

Теперь у вас есть сборка Rust с `musl`! Поскольку мы установили её в отдельную
корневую директорию, надо удостовериться в том, что система может найти
исполняемые файлы и библиотеки:

```text
$ export PATH=$PREFIX/bin:$PATH
$ export LD_LIBRARY_PATH=$PREFIX/lib:$LD_LIBRARY_PATH
```

Давайте попробуем!

```text
$ echo 'fn main() { println!("hi!"); panic!("failed"); }' > example.rs
$ rustc --target=x86_64-unknown-linux-musl example.rs
$ ldd example
        not a dynamic executable
$ ./example
hi!
thread '<main>' panicked at 'failed', example.rs:1
```

Успех! Эта программа может быть скопирована на почти любую машину с Linux с той
же архитектурой процессора и будет работать без проблем.

`cargo build` также принимает опцию `--target`, так что вы можете собирать
контейнеры как обычно. Однако, возможно вам придётся пересобрать нативные
библиотеки с `musl`, чтобы иметь возможность скомпоноваться с ними.
