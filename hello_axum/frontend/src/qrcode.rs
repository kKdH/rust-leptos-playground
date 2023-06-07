use leptos::{component, create_rw_signal, event_target, event_target_value, IntoView, RwSignal, Scope, SignalGet, SignalSet, view};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::{FileList, HtmlInputElement, ProgressEvent};

#[component]
pub fn QRCode(cx: Scope) -> impl IntoView {

    let text: RwSignal<Option<String>> = create_rw_signal(cx, None);
    let progress_max: RwSignal<f64> = create_rw_signal(cx, 0.0);
    let progress_value: RwSignal<f64> = create_rw_signal(cx, 0.0);

    let file_reader = web_sys::FileReader::new().unwrap();

    let on_load_end: Closure<dyn Fn(ProgressEvent)> = {
        let file_reader = Clone::clone(&file_reader);
        Closure::new(move |_: ProgressEvent| {
            let data: Vec<u8> = js_sys::Uint8Array::new(&file_reader.result().unwrap()).to_vec();
            let image = image::load_from_memory(&data).unwrap();
            let decoder = bardecoder::default_decoder();
            let results = decoder.decode(&image);
            results.iter().for_each(|result| {
                match result {
                    Err(error) => {
                        text.set(Some(format!("Error: {}", error)));
                    },
                    Ok(value) => {
                        text.set(Some(Clone::clone(value)));
                    },
                };
            });
        })
    };

    let on_load_progress: Closure<dyn Fn(ProgressEvent)> = {
        Closure::new(move |event: ProgressEvent| {
            progress_max.set(event.total());
            progress_value.set(event.loaded());
        })
    };

    file_reader.set_onloadend(Some(on_load_end.as_ref().unchecked_ref()));
    file_reader.set_onprogress(Some(on_load_progress.as_ref().unchecked_ref()));

    on_load_end.forget();
    on_load_progress.forget();

    view! { cx,
        <h1>"QR Code Test"</h1>
        <div>
            <label for="file">"QR Code"</label>
            <input id="file" type="file" accept="image/jpg, image/jpeg, image/png" capture="environment" on:change=move |event| {
                let input: HtmlInputElement = event_target(&event);
                let value = event_target_value(&event);
                let files: FileList = input.files().unwrap();

                text.set(None);
                progress_value.set(0.0);
                file_reader.read_as_array_buffer(&files.get(0).unwrap());
            } />
            <progress value=move || progress_value.get() max=move || progress_max.get()></progress>
            <h2>{text}</h2>
        </div>
    }
}
