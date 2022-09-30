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
    pub fluid: bool,
}

#[derive(Clone, Debug)]
pub struct Container {
    props: Props,
}

impl Component for Container {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Container { props: ctx.props().to_owned() }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        const CONTAINER_CLASS: &str = "mui-container";
        const CONTAINER_CLASS_FLUID: &str = "mui-container-fluid";
        let class = self.props.class.clone().extend(if self.props.fluid {
            CONTAINER_CLASS_FLUID
        } else {
            CONTAINER_CLASS
        });
        html! {
            <div class={class}>
                { self.props.children.clone() }
            </div>
        }
    }
}
