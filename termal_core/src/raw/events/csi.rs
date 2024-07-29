pub(crate) struct Csi {
    pub prefix: String,
    pub args: Vec<u32>,
    pub postfix: String,
}

impl Csi {
    pub fn parse(code: &str) -> Self {
        let mut code = code.chars();
        let mut prefix = String::new();
        let mut chr = None;
        for c in &mut code {
            if c.is_ascii_digit() {
                chr = Some(c);
                break;
            }
            prefix.push(c);
        }

        let mut code = chr.into_iter().chain(code);

        let mut args = String::new();
        for c in &mut code {
            if !c.is_ascii_digit() && c != ';' {
                chr = Some(c);
                break;
            }
            args.push(c);
        }

        let args: Vec<_> = args.split(';').flat_map(|a| a.parse()).collect();
        let postfix = chr.into_iter().chain(code).collect();

        if args.is_empty() {
            Self {
                prefix: postfix,
                args,
                postfix: prefix,
            }
        } else {
            Self {
                prefix,
                args,
                postfix,
            }
        }
    }
}
