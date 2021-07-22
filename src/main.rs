use yew::prelude::*;
// use yew::services::*;
use serde_derive::{Deserialize, Serialize};
use css_in_rust::style::Style;
use yew::format::Json;
use yew::services::storage::{Area, StorageService};

const KEY: &str = "toyew.store";

enum Msg {
    StoreInput(String),
    Text,
    Strikeout(usize),
    Delete(usize),
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    texts: Vec<ListItem>,
    input_val: String,
    style: Style,
    storage: StorageService,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
struct ListItem {
    text: String,
    is_striked: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "Model",
            r#"
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            .display-row {
                display: flex;
                flex-direction: row;
            }

            .to-do-item {
                margin: 0.5em;
            }
            .to-do-item:hover {
                opacity: 0.5;
                cursor: pointer;
            }

            .input-area {
                margin: 1em;
            }

            .delete-button {
                color: red;
                margin-right: 1em;
            }
            "#,
        )
        .unwrap();


        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");

        let texts = {
            if let Json(Ok(restored_model)) = storage.restore(KEY) {
                restored_model
            } else {
                Vec::new()
            }
        };
        Self {
            link,
            // count: 0,
            texts,
            input_val: "".to_string(),
            style,
            storage
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StoreInput(input) => {
                self.input_val = input;

                false
            }
            Msg::Text => {
                self.texts.push(ListItem {
                    text: self.input_val.to_string(),
                    is_striked: false,
                });
                self.input_val = "".to_string();
                self.storage.store(KEY, Json(&self.texts));

                true
            }
            Msg::Strikeout(input) => {
                let item = &self.texts[input];
                self.texts[input] = ListItem {
                    text: item.text.clone(),
                    is_striked: !item.is_striked,
                };
                self.storage.store(KEY, Json(&self.texts));

                true
            }
            Msg::Delete(input) => {
                self.texts.remove(input);
                self.storage.store(KEY, Json(&self.texts));

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.clone()>
                <h1>{" To-yew list "}</h1>
                <div class="display-row input-area">
                    <input
                        value=self.input_val.clone()
                        oninput=self.link.callback(|e: InputData| Msg::StoreInput(e.value))
                        onkeypress=self.link.batch_callback(move |e: KeyboardEvent| {
                            if e.key() == "Enter" { Some(Msg::Text) } else { None }
                        })
                    />
                    <button onclick={self.link.callback(|_| Msg::Text)}>{"add todo"}</button>
                </div>
                {for self.texts.iter().enumerate().rev().map(|(idx, item)| {
                    html!{
                        <div class="display-row to-do-item">
                            <div class="delete-button" onclick={self.link.callback(move |_| Msg::Delete(idx))}>{"✕"}</div>
                            <div
                                style={if item.is_striked == true {"text-decoration: line-through; color: red;"} else {""}}
                                onclick={self.link.callback(move |_| Msg::Strikeout(idx))}
                            >
                                {item.text.to_string()}
                            </div>
                        </div>
                    }
                })}
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
