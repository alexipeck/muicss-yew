use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[derive(Clone, Debug)]
pub struct Appbar {
    props: Props,
}

impl Component for Appbar {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Appbar {
            props: ctx.props().to_owned(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props.neq_assign(ctx.props().to_owned())
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        const APPBAR_CLASS: &str = "mui-appbar";
        let mut class = self.props.class.clone();
        class.push(APPBAR_CLASS);
        html! {
            <div class={class}>
                { self.props.children.clone() }
            </div>
        }
    }
}
