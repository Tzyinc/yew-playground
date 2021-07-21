use yew::prelude::*;

enum Msg {
    StoreInput(String),
    Text,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    texts: Vec<String>,
    input_val: String,
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
            },
            Msg::Text => {
                self.texts.push(self.input_val.to_string());
                self.input_val = "".to_string();

                true
            },
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
                <button onclick={self.link.callback(|_| Msg::Text)}>{"add todo"}</button>
                { for self.texts.iter().rev().map(|text| {html!{<div>{text}</div>}})}
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
