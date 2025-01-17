// Projection Factory
// Hey projection create (register) an Inline Projection for this view
// Hey projection create an Async Projection for this view

pub struct ViewManager {
    pub views: HashMap<String, Box<dyn View>>
}

pub enum ProjectionKind {
    Async,
    Sync
}

impl ViewManager {
    pub fn register<T>(view: T, mode: ProjectionKind) {
        // define table name from view
    }
}

pub trait View {
    fn apply(&mut self, e: &Event);
}