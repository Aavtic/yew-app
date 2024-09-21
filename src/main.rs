use yew::prelude::*;

enum Msg {
    SayHi(String),
    Submit,
}

struct Text {
    msg: String,
    greeting_msg: String,
}

impl Component for Text {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            msg: String::new(),
            greeting_msg: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::SayHi(msg) => {
                self.msg = msg;
                true
            },
            Msg::Submit => {
                self.greeting_msg = format!("Hi {}", self.msg);
                true
            }

        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <input
                    type="text"
                    oninput={ctx.link().callback(|e: InputEvent| {
                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                        Msg::SayHi(input.value())
                    })}
                    placeholder="Enter your name"
                />
                <button onclick={ctx.link().callback(|_| Msg::Submit)}>{"Submit"}</button>
                <div>{ &self.greeting_msg }</div>
            </div>
        }
    } 
}

fn main() {
    yew::Renderer::<Text>::new().render();
}

