
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Pos<T>{
    pub x: T,
    pub y: T,
}


#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Vec2<T>{
    pub dx: T,
    pub dy: T,
}
