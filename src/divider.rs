use yew::prelude::*;
use yew::html::Scope;

pub struct Divider;

impl Component for Divider {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Divider
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        const DIVIDER_CLASS: &str = "mui-divider";
        let class = Classes::from(DIVIDER_CLASS);
        html! {
            <div class={class}></div>
        }
    }
}
