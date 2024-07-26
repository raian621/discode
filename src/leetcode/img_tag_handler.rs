use html2md::{Handle, StructuredPrinter, TagHandler, TagHandlerFactory};

pub struct ImgTagHandler {}
pub struct ImgTagHandlerFactory {}

impl TagHandler for ImgTagHandler {
    fn handle(&mut self, _tag: &Handle, _printer: &mut StructuredPrinter) {}
    fn after_handle(&mut self, _printer: &mut StructuredPrinter) {}
}

impl TagHandlerFactory for ImgTagHandlerFactory {
    fn instantiate(&self) -> Box<dyn TagHandler> { Box::new(ImgTagHandler{}) }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_img_handler() {
        let html = concat!(
            "<h1>Hello world</h1>\n",
            "<img src=\"/some-link\"/>\n",
            "<p>this is a test html snippet</p>"
        );

        let markdown = html2md::parse_html_custom(html, &HashMap::from([
            ("img".to_string(), Box::new(ImgTagHandlerFactory{}) as Box<dyn TagHandlerFactory>)
        ]));

        assert_eq!(markdown, concat!(
            "Hello world\n",
            "==========\n\n",
            "this is a test html snippet"
        ));
    }
}