use nih_plug_webview::{
    DropData, DropEffect, EventStatus, HTMLSource, Key, MouseEvent, WebViewEditor,
};

use crate::CompressorPlugin;

pub fn create_editor(_plugin: &mut CompressorPlugin) -> WebViewEditor {
    // let params = plugin.params.clone();
    // let gain_value_changed = plugin.params.gain_value_changed.clone();

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
            /*
            let mut sent_from_gui = false;
            while let Ok(value) = ctx.next_event() {
                if let Ok(action) = serde_json::from_value(value) {
                    #[allow(clippy::single_match)]
                    match action {
                        Action::SetGain { value } => {
                            sent_from_gui = true;
                            setter.begin_set_parameter(&params.gain);
                            setter.set_parameter(&params.gain, value);
                            setter.end_set_parameter(&params.gain);
                        }
                    }
                } else {
                    panic!("Invalid action received from web UI.")
                }
            }

            if !sent_from_gui && gain_value_changed.swap(false, Ordering::Relaxed) {
                let data = PluginMessage::ParamChange {
                    param: "gain".to_owned(),
                    value: params.gain.value(),
                };
                let _ = ctx.send_json(json!(data));
            }
             */
        });
    editor
}
