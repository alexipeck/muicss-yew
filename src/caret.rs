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

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        const CARET_CLASS: &str = "mui-caret";
        let class =
            Classes::from(CARET_CLASS).extend(self.props.direction.map(|c| c.class(CARET_CLASS)));
        html! {
            <span class={class}></span>
        }
    }
}
