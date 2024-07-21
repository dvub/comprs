use std::{mem::discriminant, sync::atomic::Ordering};

use nih_plug::nih_log;
use nih_plug_webview::{
    DropData, DropEffect, EventStatus, HTMLSource, Key, MouseEvent, WebViewEditor,
};
use serde_json::json;

use crate::{
    params::{
        Amplitude, Message,
        Parameter::{self, *},
    },
    CompressorPlugin,
};

pub fn create_editor(plugin: &CompressorPlugin) -> WebViewEditor {
    let params = plugin.params.clone();
    let event_buffer = params.event_buffer.clone();

    let pre_amplitude = plugin.pre_amplitude.clone();
    let post_amplitude = plugin.post_amplitude.clone();

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
            let mut event_buffer_lock = event_buffer.lock().unwrap();
            let mut gui_event_buffer: Vec<Parameter> = Vec::new();
            // in the event loop, we need to do 2 basic things, as far as parameters go

            // 1. receive parameter updates (and any other events) from GUI
            while let Ok(value) = ctx.next_event() {
                if let Ok(action) = serde_json::from_value::<Message>(value) {
                    match action {
                        Message::Init => {
                            nih_log!("GUI Opened, sending initial data..");
                            // TODO:
                            // is there a nicer way ot do this?
                            let vec = vec![
                                Threshold(params.threshold.value()),
                                Ratio(params.ratio.value()),
                                KneeWidth(params.knee_width.value()),
                                AttackTime(params.attack_time.value()),
                                ReleaseTime(params.release_time.value()),
                                InputGain(params.input_gain.value()),
                                OutputGain(params.output_gain.value()),
                                DryWet(params.dry_wet.value()),
                                RmsBufferSize(params.rms_buffer_size.value()),
                                Lookahead(params.lookahead.value()),
                                RmsMix(params.lookahead.value()),
                            ];

                            for v in vec {
                                event_buffer_lock.push(v);
                            }
                        }
                        Message::ParameterUpdate(event) => {
                            let (param, value) = params.get_param(&event);
                            println!("{}, {}", param, value);
                            // todo(?)
                            // is retain() necessary
                            gui_event_buffer.retain(|d| discriminant(d) != discriminant(&event));
                            gui_event_buffer.push(event);

                            setter.begin_set_parameter(param);
                            setter.set_parameter(param, value);
                            setter.end_set_parameter(param);
                        }
                        Message::Amplitude(_) => todo!(),
                        Message::WindowClosed => println!("Window closed"),
                    }
                } else {
                    println!("Error receiving message from GUI");
                }
            }

            // 2. handle parameter updates from DAW (stuff like automation, etc)
            // these need to be sent to the GUI!!

            // remove GUI events from event buffer
            // we don't want to receive GUI events just to send them back to the GUI!!

            for event in gui_event_buffer {
                event_buffer_lock.retain(|x| discriminant(x) != discriminant(&event));
            }
            // send the remaining events to the GUI
            for event in event_buffer_lock.iter() {
                ctx.send_json(json!(event))
                    .expect("Error sending data to frontend");
            }

            let pre = pre_amplitude.load(Ordering::Relaxed);
            let post = post_amplitude.load(Ordering::Relaxed);
            let message = Amplitude::new(pre, post);
            ctx.send_json(json!(message)).expect("OH NO!");

            // once we've sent our pending updates to the GUI, we can clear our event buffer;
            event_buffer_lock.clear();
        });
    editor
}
