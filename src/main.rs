use std::collections::HashMap;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use gloo::file::callbacks::FileReader;
use gloo::file::File;
use web_sys::{js_sys, DragEvent, Event, FileList, HtmlInputElement};
use yew::html::TargetCast;
use yew::{html, Callback, Component, Context, Html};

struct FileInfo {
    name: String,
    filetype: String,
    data: Vec<u8>,
}

pub enum Mesg {
    Loaded(String, String, Vec<u8>),
    Files(Vec<File>),
}

pub struct App {
    reade: HashMap<String, FileReader>,
    files: Vec<FileInfo>,
}

impl Component for App {
    type Message = Mesg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            reade: HashMap::default(),
            files: Vec::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Mesg::Loaded(file_name, filetype, data) => {
                self.files.push(FileInfo {
                    data,
                    filetype,
                    name: file_name.clone(),
                });
                self.reade.remove(&file_name);
                true
            }
            Mesg::Files(files) => {
                for file in files.into_iter() {
                    let file_name = file.name();
                    let filetype = file.raw_mime_type();  // Corrected: Direct method for MIME type

                    let tak = {
                        let link = ctx.link().clone();
                        let file_name = file_name.clone();

                        gloo::file::callbacks::read_as_bytes(&file, move |res| {
                            link.send_message(Mesg::Loaded(
                                file_name,
                                filetype,
                                res.expect("Failed to read file"),
                            ))
                        })
                    };
                    self.reade.insert(file_name, tak);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="wrapper">
                <p id="title">{ "Upload Your Files To The Cloud" }</p>
                <label for="file-upload">
                    <div
                        id="drop-container"
                        ondrop={ctx.link().callback(|event: DragEvent| {
                            event.prevent_default();
                            let files = event.data_transfer().unwrap().files();  // Corrected: DragEvent::data_transfer
                            Self::upload_files(files)
                        })}
                        ondragover={Callback::from(|event: DragEvent| {
                            event.prevent_default();
                        })}
                        ondragenter={Callback::from(|event: DragEvent| {
                            event.prevent_default();
                        })}
                    >
                        <i class="fa fa-cloud-upload"></i>
                        <p>{"Drop your images here or click to select"}</p>
                    </div>
                </label>
                <input
                    id="file-upload"
                    type="file"
                    accept="image/*,video/*"
                    multiple={true}
                    onchange={ctx.link().callback(move |e: Event| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        Self::upload_files(input.files())
                    })}
                />
                <div id="preview-area">
                    { for self.files.iter().map(Self::view_file) }
                </div>
            </div>
        }
    }
}

impl App {
    fn view_file(file: &FileInfo) -> Html {
        html! {
            <div class="preview-tile">
                <p class="preview-name">{ format!("{}", file.name) }</p>
                <div class="preview-media">
                    if file.filetype.contains("image") {
                        <img src={format!("data:{};base64,{}", file.filetype, STANDARD.encode(&file.data))} />
                    } else if file.filetype.contains("video") {
                        <video controls={true}>
                            <source src={format!("data:{};base64,{}", file.filetype, STANDARD.encode(&file.data))} type={file.filetype.clone()} />
                        </video>
                    }
                </div>
            </div>
        }
    }

    fn upload_files(files: Option<FileList>) -> Mesg {
        let mut res = Vec::new();
        if let Some(files) = files {
            let files = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from);
            res.extend(files);
        }
        Mesg::Files(res)
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
