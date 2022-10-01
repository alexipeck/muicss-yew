use yew::prelude::*;
use yewtil::NeqAssign;
use yew::html::Scope;

prop_enum! {
    Color {
        Primary => "primary",
        Danger => "danger",
        Dark => "dark",
        Accent => "accent",
    }
}

prop_enum! {
    Size {
        Small => "small",
        Large => "large",
    }
}

prop_enum! {
    Variant {
        Flat => "flat",
        Raised => "raised",
        Fab => "fab",
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub variant: Option<Variant>,
    #[prop_or_default]
    pub disabled: bool,
}

#[derive(Clone, Debug)]
pub struct Button {
    props: Props,
}

impl Component for Button {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Button { props: ctx.props().to_owned() }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props.neq_assign(ctx.props().to_owned())
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        const BTN_CLASS: &str = "mui-btn";
        let mut class = self
            .props
            .class
            .clone();
        class.push(BTN_CLASS);
        class.push(self.props.color.map(|c| c.class(BTN_CLASS)));
        class.push(self.props.size.map(|c| c.class(BTN_CLASS)));
        class.push(self.props.variant.map(|c| c.class(BTN_CLASS)));

        html! {
            <button class={class}
                onclick={&self.props.onclick}
                disabled={self.props.disabled}>
                { self.props.children.clone() }
            </button>
        }
    }
}
