#[derive(Clone)]
pub struct UiResolutionContext {
    pub stack_id: Arc<str>,
    pub layout_stack: Arc<Vec<LayoutNode>>,
    pub metadata: Arc<ResolvedMetadata>,
    pub pipeline: Arc<ResponsePipeline>,   
}