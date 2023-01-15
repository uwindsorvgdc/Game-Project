
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Pos<T>{
    pub x: T,
    pub y: T,
}


#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Rectangle<T>{
    pub corner1: Pos<T>,
    pub corner2: Pos<T>
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Rgba{
    pub red: u8,
    pub blue: u8,
    pub green: u8,
}



pub trait Graphics{
    type Scalar: Copy + Add + Eq;
    type Error: std::error::Error;
    fn dimensions(&self) -> Rectangle<Self::Scalar>;
    fn draw_rect(&mut self, corner1: Pos<Self::Scalar>, corner2: Pos<Self::Scalar>, color: Rgba) -> Result<(),Self::Error>;
    fn draw_square(&mut self, corner1: Pos<Self::Scalar>, side: Self::Scalar, color: Rgba) -> Result<(),Self::Error>{
        self.draw_rect(corner1, Pos{x: corner1.x+side, y: corner1.y+side},color)
    }
    fn draw_line(&mut self, start: Pos<Self::Scalar>, end: Pos<Self::Scalar>, color: Rgba) -> Result<(),Self::Error>;
    fn draw_point(&mut self, pos: Pos<Self::Scalar>, color: Rgba)  -> Result<(),Self::Error>;
    fn draw_elipse(&mut self, center: Pos<Self::Scalar>, rada: Self::Scalar, radb: Self::Scalar, color: Rgba)  -> Result<(),Self::Error>;
    fn draw_circle(&mut self, center: Pos<Self::Scalar>, rad: Self::Scalar, color: Rgba) -> Result<(),Self::Error>{
        self.draw_elipse(center,rad,rad)
    }
}