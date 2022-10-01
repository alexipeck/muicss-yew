use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputType {
    Email,
    Password,
    Search,
    Tel,
    Text,
    Url,
}

impl Default for InputType {
    fn default() -> Self {
        Self::Text
    }
}

impl InputType {
    fn input_type(&self) -> &'static str {
        match self {
            InputType::Email => "email",
            InputType::Password => "password",
            InputType::Search => "search",
            InputType::Tel => "tel",
            InputType::Text => "text",
            InputType::Url => "url",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onchange: Callback<Event>,
    #[prop_or_default]
    pub input_type: InputType,
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
pub struct Input {
    props: Props,
}

impl Component for Input {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Input {
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
        const TEXTFIELD_CLASS: &str = "mui-textfield";
        const FLOAT_LABEL_CLASS: &str = "mui-textfield--float-label";
        const INVALID_CLASS: &str = "mui--is-invalid";
        let mut class = self.props.class.clone();
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
                <input type={self.props.input_type.input_type()}
                    onchange={&self.props.onchange}
                    disabled={self.props.disabled}
                    placeholder={self.props.placeholder.to_owned()}
                    value={self.props.value.to_owned()} />
                { label }
            </div>
        }
    }
}
