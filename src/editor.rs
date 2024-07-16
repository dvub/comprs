use std::mem::discriminant;

use nih_plug_webview::{
    DropData, DropEffect, EventStatus, HTMLSource, Key, MouseEvent, WebViewEditor,
};
use serde_json::json;

use crate::{params::ParameterEvent, CompressorPlugin};

pub fn create_editor(plugin: &mut CompressorPlugin) -> WebViewEditor {
    let params = plugin.compressor.params.clone();
    let event_buffer = plugin.compressor.params.event_buffer.clone();

    let size = (750, 500);

    #[cfg(debug_assertions)]
    let src = HTMLSource::URL("http://localhost:3000".to_owned());
    #[cfg(debug_assertions)]
    let mut editor = WebViewEditor::new(src, size);

    #[cfg(not(debug_assertions))]
    let mut editor = nih_plug_webview::editors::editor_with_frontend_dir(
        "D:\\projects\\rust\\comprs\\gui\\out".into(),
        size,
        None,
    );
    editor = editor
        .with_developer_mode(true)
        .with_keyboard_handler(move |event| {
            println!("keyboard event: {event:#?}");
            event.key == Key::Escape
        })
        .with_mouse_handler(|event| match event {
            MouseEvent::DragEntered { .. } => {
                println!("drag entered");
                EventStatus::AcceptDrop(DropEffect::Copy)
            }
            MouseEvent::DragMoved { .. } => {
                println!("drag moved");
                EventStatus::AcceptDrop(DropEffect::Copy)
            }
            MouseEvent::DragLeft => {
                println!("drag left");
                EventStatus::Ignored
            }
            MouseEvent::DragDropped { data, .. } => {
                if let DropData::Files(files) = data {
                    println!("drag dropped: {:?}", files);
                }
                EventStatus::AcceptDrop(DropEffect::Copy)
            }
            _ => EventStatus::Ignored,
        })
        .with_event_loop(move |ctx, setter, _window| {
            let mut gui_event_buffer: Vec<ParameterEvent> = Vec::new();
            // in the event loop, we need to do 2 basic things, as far as parameters go

            // 1. receive parameter updates (and any other events) from GUI
            while let Ok(value) = ctx.next_event() {
                if let Ok(action) = serde_json::from_value(value) {
                    let (param, value) = params.get_param(&action);

                    gui_event_buffer.retain(|event| discriminant(event) != discriminant(&action));
                    gui_event_buffer.push(action);

                    setter.begin_set_parameter(param);
                    setter.set_parameter(param, value);
                    setter.end_set_parameter(param);
                } else {
                    println!("Error receiving message from GUI");
                }
            }

            // 2. handle parameter updates from DAW (stuff like automation, etc)
            // these need to be sent to the GUI!!

            // remove GUI events from event buffer
            // we don't want to receive GUI events just to send them back to the GUI!!
            let mut event_buffer_lock = event_buffer.lock().unwrap();
            for event in gui_event_buffer {
                event_buffer_lock.retain(|x| discriminant(x) != discriminant(&event));
            }
            // send the remaining events to the GUI
            for event in event_buffer_lock.iter() {
                ctx.send_json(json!(event))
                    .expect("Error sending data to frontend");
            }
            // once we've sent our pending updates to the GUI, we can clear our event buffer;
            event_buffer_lock.clear();
        });
    editor
}
