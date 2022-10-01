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
    pub xs: Option<u8>,
    #[prop_or_default]
    pub sm: Option<u8>,
    #[prop_or_default]
    pub md: Option<u8>,
    #[prop_or_default]
    pub lg: Option<u8>,
    #[prop_or_default]
    pub xl: Option<u8>,
    #[prop_or_default]
    pub xs_offset: Option<u8>,
    #[prop_or_default]
    pub sm_offset: Option<u8>,
    #[prop_or_default]
    pub md_offset: Option<u8>,
    #[prop_or_default]
    pub lg_offset: Option<u8>,
    #[prop_or_default]
    pub xl_offset: Option<u8>,
}

impl Props {
    fn responsive(&self) -> [(&str, Option<u8>, Option<u8>); 5] {
        [
            ("xs", self.xs, self.xs_offset),
            ("sm", self.sm, self.sm_offset),
            ("md", self.md, self.md_offset),
            ("lg", self.lg, self.lg_offset),
            ("xl", self.xl, self.xl_offset),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct Col {
    props: Props,
}

impl Component for Col {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Col { props: ctx.props().to_owned() }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props.neq_assign(ctx.props().to_owned())
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        const COL_CLASS: &str = "mui-col";
        let mut class = self.props.class.clone();
        class.push(COL_CLASS);
        for (prefix, value, offset_value) in &self.props.responsive() {
            if let Some(value) = value {
                class.push(&format!("{}-{}-{}", COL_CLASS, prefix, value));
            }
            if let Some(value) = offset_value {
                class.push(&format!("{}-{}-offset-{}", COL_CLASS, prefix, value));
            }
        }
        html! {
            <div class={class}>
                { self.props.children.clone() }
            </div>
        }
    }
}
