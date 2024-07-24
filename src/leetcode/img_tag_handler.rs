use html2md::{Handle, StructuredPrinter, TagHandler, TagHandlerFactory};

pub struct ImgTagHandler {}
pub struct ImgTagHandlerFactory {}

impl TagHandler for ImgTagHandler {
    fn handle(&mut self, _tag: &Handle, _printer: &mut StructuredPrinter) {}
    fn after_handle(&mut self, _printer: &mut StructuredPrinter) {}
    fn skip_descendants(&self) -> bool { true }
}

impl TagHandlerFactory for ImgTagHandlerFactory {
    fn instantiate(&self) -> Box<dyn TagHandler> { Box::new(ImgTagHandler{}) }
}