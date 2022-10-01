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
    pub onchange: Callback<Event>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub invalid: bool,
    #[prop_or_default]
    pub floating_label: bool,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub placeholder: String,
}

#[derive(Clone, Debug)]
pub struct Textarea {
    props: Props,
}

pub enum Msg {
    Change(Event),
}

impl Component for Textarea {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Textarea { props: ctx.props().to_owned() }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props.neq_assign(ctx.props().to_owned())
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        const TEXTFIELD_CLASS: &str = "mui-textfield";
        const FLOAT_LABEL_CLASS: &str = "mui-textfield--float-label";
        const INVALID_CLASS: &str = "mui--is-invalid";

        let mut class = self
            .props
            .class
            .clone();
        class.push(TEXTFIELD_CLASS);
        if self.props.invalid {
            class.push(INVALID_CLASS);
        }
        if self.props.floating_label {
            class.push(FLOAT_LABEL_CLASS);
        }
        let label = if self.props.children.is_empty() {
            Html::default()
        } else {
            html! {
                <label>
                    { self.props.children.clone() }
                </label>
            }
        };

        html! {
            <div class={class}>
                <textarea onchange={&self.props.onchange}
                    disabled={self.props.disabled}
                    placeholder={self.props.placeholder.to_owned()}
                    value={self.props.value.to_owned()} />
                { label }
            </div>
        }
    }
}
