use serde::{Deserialize, Serialize};
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

#[derive(Serialize, Deserialize)]
pub struct JsonEntity<T: Serialize> {
    result: T,
}

impl<T: Serialize> JsonEntity<T> {
    pub fn new(t: T) -> Self {
        Self { result: t }
    }

    pub fn to_json(self) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(&self)?)
    }

    pub fn print_json(self, no_colors: bool) -> anyhow::Result<()> {
        let json = self.to_json()?;

        if no_colors {
            println!("{}", json);

            return Ok(());
        } else {
            let ps = SyntaxSet::load_defaults_newlines();
            let ts = ThemeSet::load_defaults();

            let syntax = ps.find_syntax_by_extension("json").unwrap();
            let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

            for line in LinesWithEndings::from(&json) {
                let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
                let escaped = as_24_bit_terminal_escaped(&ranges[..], false);

                print!("{}", escaped);
            }

            Ok(())
        }
    }
}
