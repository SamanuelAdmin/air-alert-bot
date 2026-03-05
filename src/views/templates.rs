/*

Here is some features for template rendering,
managing and formatting.
Templates are used in vies.

*/


use tera::Tera;
use tera::Context;


pub struct TemplatesManager {
    tera: Tera
}

impl TemplatesManager {
    pub fn new(templates_dir: String) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            tera: Tera::new(&templates_dir)?
        })
    }

    pub fn render_template(&self, template_name: &str, context: &Context)
        -> Result<String, Box<dyn std::error::Error>> {
        Ok(self.tera.render(
            template_name, context
        )?)
    }
}
