pub(crate) struct Osc<'a> {
    pub args: Vec<u32>,
    pub data: &'a str,
}

impl<'a> Osc<'a> {
    pub fn parse(mut code: &'a str) -> Self {
        // Might not work in some special cases (e.g. clipboard paste with
        // base64 representantion containing only digits)
        let mut args = vec![];
        loop {
            if code.starts_with(';') {
                args.push(0);
                code = &code[1..];
                continue;
            }
            if !code.starts_with(|c: char| c.is_ascii_digit()) {
                return Self { args, data: code };
            }
            let Some((i, c)) =
                code.char_indices().find(|(_, c)| !c.is_ascii_digit())
            else {
                args.push(code.parse::<u32>().ok().unwrap_or_default());
                continue;
            };
            if c != ';' {
                return Self { args, data: code };
            }
            args.push(code[..i].parse::<u32>().unwrap_or_default());
            code = &code[i + 1..];
        }
    }
}
