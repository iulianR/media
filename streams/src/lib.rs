use std::any::Any;

pub trait MediaStream: Any + Send {
    fn as_any(&self) -> &Any;
    fn as_mut_any(&mut self) -> &mut Any;
}

pub struct DummyMediaOutput;
impl MediaOutput for DummyMediaOutput {
    fn add_stream(&mut self, _stream: Box<MediaStream>) {}
}

/// This isn't part of the webrtc spec; it's a leaky abstaction while media streams
/// are under development and example consumers need to be able to inspect them.
pub trait MediaOutput: Send {
    fn add_stream(&mut self, stream: Box<MediaStream>);
}
