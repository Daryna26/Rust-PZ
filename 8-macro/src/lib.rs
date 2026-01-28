use std::collections::BTreeMap;

/// Declarative btreemap! macro
#[macro_export]
macro_rules! btreemap {
    () => {
        ::std::collections::BTreeMap::new()
    };
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = ::std::collections::BTreeMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}
