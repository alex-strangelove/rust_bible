// fn main() {
//     // Traits
//     let a = Area { x: 10, y: 20 };
//     print_debug(a.clone());
//     print_debug2(a.clone());
//     print_debug3(a.clone());

//     // Trait bounds
//     let my_string = String::from("Hello World!");
//     let word = first_word(&my_string);
//     println!("{}", word);

//     // High ranked trait bounds
//     let formatter = SimpleFormatter;

//     let s1 = "Hello";
//     let s2 = String::from("World");

//     let format_fn = apply_format(formatter);

//     {
//         let s3: String = String::from("World");
//         println!("{}", format_fn(&s3));
//     }
// }

mod traits;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_debug_formatter() {
        #[derive(Debug, Clone)]
        struct Point {
            x: i32,
        }

        let point = Point { x: 10 };

        assert_eq!(point.x, 10);

        assert_eq!(
            "value: Point { x: 10 }",
            traits::trait_fn::format_debug1(point.clone())
        );
        assert_eq!(
            "value: Point { x: 10 }",
            traits::trait_fn::format_debug1(point.clone())
        );
        assert_eq!(
            "value: Point { x: 10 }",
            traits::trait_fn::format_debug2(point.clone())
        );
    }
}
