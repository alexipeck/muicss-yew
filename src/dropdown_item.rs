use yew::prelude::*;
use yewtil::NeqAssign;
use yew::html::Scope;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or("javascript:void(0);".to_owned())]
    pub link: String,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[derive(Clone, Debug)]
pub struct DropdownItem {
    props: Props,
}

impl Component for DropdownItem {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        DropdownItem { props: ctx.props().to_owned() }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <li>
                <a href={self.props.link.as_str()}
                    onclick={&self.props.onclick}>
                    { self.props.children.clone() }
                </a>
            </li>
        }
    }
}
