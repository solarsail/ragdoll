use std::sync::mpsc;
use multiqueue;
use sdl2::event::Event;


pub struct Context {
    events: multiqueue::BroadcastReceiver<Event>,
    render_q: mpsc::Sender<RenderPiece>,
}
