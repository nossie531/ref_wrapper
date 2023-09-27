use super::drop_tracer::DropTracer;

pub struct DropItem<'a> {
    tracer: &'a DropTracer,
}

impl<'a> DropItem<'a> {
    pub(super) fn new(tracer: &'a DropTracer) -> Self {
        Self { tracer }
    }
}

impl Drop for DropItem<'_> {
    fn drop(&mut self) {
        self.tracer.drop_item()
    }
}
