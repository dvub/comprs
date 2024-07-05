use nih_plug_webview::{
    DropData, DropEffect, EventStatus, HTMLSource, Key, MouseEvent, WebViewEditor,
};

use crate::CompressorPlugin;

pub fn create_editor(plugin: &mut CompressorPlugin) -> WebViewEditor {
    let params = plugin.compressor.params.clone();
    let changed = plugin.compressor.params.changed_params.clone();

    let size = (750, 500);

    #[cfg(debug_assertions)]
    let src = HTMLSource::URL("http://localhost:3000".to_owned());
    #[cfg(debug_assertions)]
    let mut editor = WebViewEditor::new(src, size);

    #[cfg(not(debug_assertions))]
    let mut editor = nih_plug_webview::editors::editor_with_frontend_dir(
        "D:\\projects\\rust\\next-gain\\gui\\out".into(),
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
            // receive incoming events from GUI
            while let Ok(value) = ctx.next_event() {
                if let Ok(action) = serde_json::from_value(value) {
                    let (param, value) = params.get_param(action);
                    setter.begin_set_parameter(param);
                    setter.set_parameter(param, value);
                    setter.end_set_parameter(param);
                } else {
                    println!("Received a funky guy");
                }
            }
            // now, send new events to GUI
            // todo
            println!("{}", changed);
            for p in changed.lock().unwrap().iter() {
                println!("{:?}", p);
            }
            // once we've consumed the values, we can clear the vec until later
            changed.lock().unwrap().clear();
        });
    editor
}
