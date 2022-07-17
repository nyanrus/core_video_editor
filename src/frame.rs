struct Frame{
    w:u32,
    h:u32,
    pix_vec:Vec<u8>
} // RGBA
struct Settings{}
trait InOut {
    fn get_frame(&self, settings:Settings) -> &Self;
}
trait In : InOut {
    fn get_frame(settings:Settings) -> Self;
}
trait Out:InOut {
    fn get_frame(&self,settings:Settings);
}



// fn a() {
//     let a = frame{};
//     a.get_frame(settings{});
//     frame::get_frame(settings);
// }