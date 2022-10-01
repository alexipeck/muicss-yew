use yew::prelude::*;
use yewtil::NeqAssign;
use yew::html::Scope;

prop_enum! {
    Direction {
        Up => "up",
        Right => "right",
        Left => "left",
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub direction: Option<Direction>,
}

#[derive(Clone, Debug)]
pub struct Caret {
    props: Props,
}

impl Component for Caret {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Caret { props: ctx.props().to_owned() }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props.neq_assign(ctx.props().to_owned())
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        const CARET_CLASS: &str = "mui-caret";
        let mut class = Classes::from(CARET_CLASS);
        class.push(self.props.direction.map(|c| c.class(CARET_CLASS)));
        html! {
            <span class={class}></span>
        }
    }
}
