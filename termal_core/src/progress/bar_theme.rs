pub type DefaultBarTheme = ();

pub trait BarTheme {
    /// The default theme for spaces.
    fn reset(&self) -> &str;

    /// The style of task name.
    fn task(&self) -> &str;

    /// The style of the percentage.
    fn percent(&self) -> &str;

    /// Style of the progress bar start.
    fn start(&self) -> &str;

    /// Style of the progress bar full.
    fn full(&self) -> &str;

    /// Style of the progress bar thumb.
    fn thumb(&self) -> &str;

    /// Style of the progress bar empty.
    fn empty(&self) -> &str;

    /// Style of the progress bar end.
    fn end(&self) -> &str;

    /// Style of the time.
    fn time(&self) -> &str;

    /// Style of the info.
    fn info(&self) -> &str;
}

impl BarTheme for () {
    fn reset(&self) -> &str {
        "\x1b[0m"
    }

    fn task(&self) -> &str {
        "\x1b[97m"
    }

    fn percent(&self) -> &str {
        "\x1b[96m"
    }

    fn start(&self) -> &str {
        "[\x1b[93m"
    }

    fn full(&self) -> &str {
        "="
    }

    fn thumb(&self) -> &str {
        "\x1b[97;1m#\x1b[0;90m"
    }

    fn empty(&self) -> &str {
        "-"
    }

    fn end(&self) -> &str {
        "\x1b[0m]"
    }

    fn time(&self) -> &str {
        "\x1b[95m"
    }

    fn info(&self) -> &str {
        ""
    }
}
