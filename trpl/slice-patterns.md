% Шаблоны `match` для срезов

Если вы хотите в качестве шаблона для сопоставления использовать срез или
массив, то вы можете использовать `&` и активировать возможность
`slice_patterns`:

```rust
#![feature(slice_patterns)]

fn main() {
    let v = vec!["match_this", "1"];

    match &v[..] {
        ["match_this", second] => println!("The second element is {}", second),
        _ => {},
    }
}
```

Отключаемая возможность `advanced_slice_patterns` позволяет использовать `..`,
чтобы обозначить любое число элементов в шаблоне. Этот символ подстановки можно
использовать в массиве один раз. Если перед `..` есть идентификатор, результат
среза будет связан с этим именем. Например:

```rust
#![feature(advanced_slice_patterns, slice_patterns)]

fn is_symmetric(list: &[u32]) -> bool {
    match list {
        [] | [_] => true,
        [x, inside.., y] if x == y => is_symmetric(inside),
        _ => false
    }
}

fn main() {
    let sym = &[0, 1, 4, 2, 4, 1, 0];
    assert!(is_symmetric(sym));

    let not_sym = &[0, 1, 7, 2, 4, 1, 0];
    assert!(!is_symmetric(not_sym));
}
```
