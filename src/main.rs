use yew::prelude::*;

enum Msg {
   Bold,
   Italic,
   Update(String)
}

struct App {
    input_value: String,
    text: String,
    html: Html,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input_value: String::new(),
            text: String::new(),
            html: Html::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::Bold => {
                self.text = self.input_value.clone();
                self.html = html! {
                    <b>{self.text.clone()}</b>
                };
                true
            },
            Msg::Italic => {
                self.text = self.input_value.clone();
                self.html = html! {
                    <i>{self.text.clone()}</i>
                };
                true
            },
            Msg::Update(msg) => {
                self.input_value = msg;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div> 
                <textarea 
                rows="4" 
                cols="50"
                value={self.input_value.clone()}
                oninput={
                    link.callback(|e: InputEvent| {
                        let textarea = e.target_unchecked_into::<web_sys::HtmlTextAreaElement>();
                        Msg::Update(textarea.value())
                    })
                }> 
                {self.input_value}
                </textarea>
            <br/>
            <button onclick={link.callback(|_| Msg::Bold)}>
            {"Bold"}
            </button>
            <button onclick={link.callback(|_| Msg::Italic)}>
            {"Italic"}
            </button>
            <br/>
            {self.html.clone()}
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

