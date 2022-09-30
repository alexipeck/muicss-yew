use yew::prelude::*;
use yewtil::NeqAssign;
use yew::html::Scope;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[derive(Clone, Debug)]
pub struct Panel {
    props: Props,
}

impl Component for Panel {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Panel { props: ctx.props().to_owned() }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        const PANEL_CLASS: &str = "mui-panel";
        let class = self.props.class.clone().extend(PANEL_CLASS);
        html! {
            <div class={class}>
                { self.props.children.clone() }
            </div>
        }
    }
}
