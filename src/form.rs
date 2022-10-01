use yew::prelude::*;
use yewtil::NeqAssign;

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
        Form {
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
        const FORM_CLASS: &str = "mui-form";
        const FORM_CLASS_INLINE: &str = "mui-form--inline";
        let mut class = self.props.class.clone();
        class.push(if self.props.inline {
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
