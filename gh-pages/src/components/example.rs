use muicss_yew::panel::Panel;
use yew::prelude::*;
use yew::{Component, Context, Html};
use yew_prism::Prism;
use yewtil::NeqAssign;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub code: String,
    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, Debug)]
pub struct Example {
    props: Props,
}

impl Component for Example {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Example {
            props: ctx.props().to_owned(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props.neq_assign(ctx.props().to_owned())
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <Panel>
                <div>
                    { self.props.children.clone() }
                </div>
                <Prism code={self.props.code.clone()} language="rust" />
            </Panel>
        }
    }
}
