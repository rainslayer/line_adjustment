pub fn transform(input: &str, line_width: u32) -> String {
    let mut result = String::new();
    let mut iter = input.split_whitespace().peekable();
    let line_width = usize::try_from(line_width).unwrap();

    while iter.peek().is_some() {
        let mut current_line_length = 0;
        let mut line = iter
            .clone()
            .take_while(|s| {
                if current_line_length + s.len() + 1 <= line_width {
                    if current_line_length > 0 {
                        current_line_length += 1;
                    }

                    current_line_length += s.len();
                    iter.next();
                    return true;
                }

                false
            })
            .map(str::to_string)
            .collect::<Vec<String>>();

        if result.len() > 0 {
            result.push_str("\n");
        }

        let num_words = line.len();

        if num_words > 1 {
            let no_of_spaces = line_width - current_line_length + (num_words - 1);
            let spaces_between_words = no_of_spaces / (num_words - 1);
            let extra_spaces = no_of_spaces % (num_words - 1);

            for i in 0..extra_spaces {
                let idx = i % (num_words - 1);
                line[idx].push_str(" ");
            }
            let joined_string = line.join(&" ".repeat(spaces_between_words));
            result.push_str(&joined_string);
        } else {
            let joined_string = line.join("");
            result.push_str(&format!("{:width$}", joined_string, width = line_width));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::transform;

    #[test]
    fn simple() {
        let test_cases = [
            ("", 5, ""),
            ("test", 5, "test "),
            ("Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua", 12,
             "Lorem  ipsum\ndolor    sit\namet        \nconsectetur \nadipiscing  \nelit  sed do\neiusmod     \ntempor      \nincididunt  \nut labore et\ndolore magna\naliqua      "),
            ("Lorem     ipsum    dolor", 17, "Lorem ipsum dolor"),
        ];

        for &(input, line_width, expected) in &test_cases {
            println!("input: '{}'", input);
            assert_eq!(transform(input, line_width), expected);
        }
    }
}
