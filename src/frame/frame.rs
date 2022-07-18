pub struct Frame{
    pub w:u32,
    pub h:u32,
    pub pix_vec:Vec<u8>
} // RGBA

pub struct Settings{}

pub trait FrameInterface {}

pub trait FrameInput<T>:FrameInterface {
    fn get_frame_in(&self,settings:&T) -> Option<Frame>;
}

pub trait FrameOutput<T>:FrameInterface {
    fn give_frame_out(&self,f:Frame,settings:&T) -> bool;
}
pub trait FrameInOut<T>:FrameInterface {
    fn get_frame_inout(&self,f:Frame,settings:&T) -> Option<Frame>;
}

// fn a() {
//     let a = frame{};
//     a.get_frame(settings{});
//     frame::get_frame(settings);
// }