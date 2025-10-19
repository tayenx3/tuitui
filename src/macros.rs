#[macro_export]
macro_rules! span {
    ($content:expr) => {
        $crate::components::text::TextSpan::new($content)
    };
    
    ($content:expr, $($method:ident $($arg:expr)*),*) => {
        {
            let mut span = $crate::components::text::TextSpan::new($content);
            $(span.$method($($arg)*);)*
            span
        }
    };
}

#[macro_export]
macro_rules! text {
    () => {
        $crate::components::Text::new()
    };
    
    ($($span:expr),* $(,)?) => {
        {
            let mut text = $crate::components::text::Text::new();
            $(
                text.span_from($span);
            )*
            text
        }
    };
}