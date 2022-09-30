use yew::prelude::*;
use yewtil::NeqAssign;
use yew::html::Scope;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub inline: bool,
}

pub struct Form {
    props: Props,
}

impl Component for Form {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Form { props: ctx.props().to_owned() }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        const FORM_CLASS: &str = "mui-form";
        const FORM_CLASS_INLINE: &str = "mui-form--inline";
        let class = self.props.class.clone().extend(if self.props.inline {
            FORM_CLASS_INLINE
        } else {
            FORM_CLASS
        });
        html! {
            <form class={class}>
                { self.props.children.clone() }
            </form>
        }
    }
}
