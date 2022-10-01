use yew::prelude::*;

pub struct Divider;

impl Component for Divider {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Divider
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        const DIVIDER_CLASS: &str = "mui-divider";
        let class = Classes::from(DIVIDER_CLASS);
        html! {
            <div class={class}></div>
        }
    }
}
