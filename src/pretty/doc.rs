use crate::pretty::part::PrettyDoc;

pub struct Doc(pub(crate) PrettyDoc);

pub struct RenderConfig {
    width: usize,
}

impl From<usize> for RenderConfig {
    fn from(width: usize) -> Self {
        RenderConfig { width }
    }
}

impl Doc {
    pub fn render(self) -> String {
        self.render_with_config(RenderConfig { width: 80 })
    }

    pub fn render_with_config(self, config: impl Into<RenderConfig>) -> String {
        let config = config.into();
        let mut vec = Vec::new();
        self.0
            .render(config.width, &mut vec)
            .expect("It should be impossible to get an io::Error when rendering into a Vec");
        String::from_utf8_lossy(&vec).to_string()
    }
}
