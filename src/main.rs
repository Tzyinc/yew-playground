use yew::prelude::*;
use yew::services::*;

enum Msg {
    StoreInput(String),
    Text,
    Strikeout(usize),
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    texts: Vec<ListItem>,
    input_val: String,
}

#[derive(Clone)]
struct ListItem {
    text: String,
    is_striked: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            // count: 0,
            texts: Vec::new(),
            input_val: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // Msg::AddOne => {
            //     self.count += 1;
            //     // the value has changed so we need to
            //     // re-render for it to appear on the page
            //     true
            // },
            Msg::StoreInput(input) => {
                self.input_val = input;

                false
            }
            Msg::Text => {
                self.texts.push(ListItem { text: self.input_val.to_string(), is_striked: false });
                self.input_val = "".to_string();

                true
            }
            Msg::Strikeout(input) => {
                let item = &self.texts[input];
                self.texts[input] = ListItem {
                    text: item.text.clone(),
                    is_striked: !item.is_striked
                };

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
            <div>
                <input
                    value=self.input_val.clone()
                    oninput=self.link.callback(|e: InputData| Msg::StoreInput(e.value))
                    onkeypress=self.link.batch_callback(move |e: KeyboardEvent| {
                        if e.key() == "Enter" { Some(Msg::Text) } else { None }
                    })
                />
                // return if value == 5 { success } else { failure }.
                <button onclick={self.link.callback(|_| Msg::Text)}>{"add todo"}</button>
                {for self.texts.iter().enumerate().rev().map(|(idx, item)| {
                    html!{
                        <div 
                          style={if item.is_striked == true {"text-decoration: line-through; color: red;"} else {""}}
                          onclick={self.link.callback(move |_| Msg::Strikeout(idx))}
                        >
                            {format!("{}{}{}", idx, ". ",  item.text)}
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
