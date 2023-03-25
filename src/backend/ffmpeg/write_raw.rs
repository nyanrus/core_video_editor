use ffmpeg::{codec, encoder, format};
use ffmpeg_next as ffmpeg;

fn write_video_raw(output: format::context::Output) {
    match format::output(&"./a.mp4") {
        Ok(mut o) => {
            let stream = o
                .add_stream(encoder::find(codec::Id::H264).unwrap())
                .unwrap();
            let vid = codec::Context::from_parameters(stream.parameters())
                .unwrap()
                .encoder()
                .video()
                .unwrap();
        }
        Err(_) => todo!(),
    }
}
