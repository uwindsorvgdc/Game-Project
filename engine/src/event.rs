
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Event{
    KeyPressed(KeyCode),
    KeyReleased(KeyCode),
    MouseMove(Pos<f32>),
    MouseButtonPressed(u32),
    MouseButtonReleased(u32),
    MouseScroll(f32),
    
}