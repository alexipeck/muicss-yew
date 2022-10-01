use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onchange: Callback<Event>,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub value: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Checkbox {
    props: Props,
}

impl Component for Checkbox {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Checkbox {
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
        const CHECKBOX_CLASS: &str = "mui-checkbox";
        let mut class = self.props.class.clone();
        class.push(CHECKBOX_CLASS);
        let value = if let Some(value) = self.props.value.clone() {
            value
        } else {
            "on".to_string()
        };
        html! {
            <div class={class}>
                <label>
                    <input type="checkbox"
                        checked={self.props.checked}
                        onchange={&self.props.onchange}
                        disabled={self.props.disabled}
                        value={value} />
                    { self.props.children.clone() }
                </label>
            </div>
        }
    }
}
