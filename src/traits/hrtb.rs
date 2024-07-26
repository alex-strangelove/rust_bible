// Trait bounds
#[allow(dead_code)]
pub fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}

// High ranked trait bounds
trait Formatter {
    fn format<T: std::fmt::Display>(&self, value: T) -> String;
}

struct SimpleFormatter;

impl Formatter for SimpleFormatter {
    fn format<T: std::fmt::Display>(&self, value: T) -> String {
        format!("Value: {}", value)
    }
}

#[allow(dead_code)]
fn apply_format<F>(formatter: F) -> impl for<'a> Fn(&str) -> String
where
    F: Formatter,
{
    move |s| formatter.format(s)
}
