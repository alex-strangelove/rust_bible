// Traits
#[allow(dead_code)]
pub fn format_debug1<T: std::fmt::Debug>(input: T) -> String {
    format!("value: {:?}", input)
}

#[allow(dead_code)]
pub fn format_debug2<T>(input: T) -> String
where
    T: std::fmt::Debug,
{
    format!("value: {:?}", input)
}

#[allow(dead_code)]
pub fn format_debug3(input: impl std::fmt::Debug) -> String {
    format!("value: {:?}", input)
}
