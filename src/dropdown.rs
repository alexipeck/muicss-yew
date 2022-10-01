use crate::{
    button::{Color, Size, Variant},
    caret::{Caret, Direction},
    dropdown_item::DropdownItem,
};
use yew::prelude::*;
use yewtil::NeqAssign;
use yew::html::Scope;

prop_enum! {
    Placement {
        Up => "up",
        Right => "right",
        Left => "left",
    }
}

impl Placement {
    fn direction(self) -> Direction {
        match self {
            Placement::Up => Direction::Up,
            Placement::Right => Direction::Right,
            Placement::Left => Direction::Left,
        }
    }
}

prop_enum! {
    Alignment {
        Right => "right",
        Bottom => "bottom",
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: ChildrenWithProps<DropdownItem>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub variant: Option<Variant>,
    #[prop_or_default]
    pub placement: Option<Placement>,
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub disabled: bool,
}

pub struct Dropdown {
    props: Props,
}

impl Component for Dropdown {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Dropdown { props: ctx.props().to_owned() }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props.neq_assign(ctx.props().to_owned())
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        const DROPDOWN_CLASS: &str = "mui-dropdown";
        let mut class = self
            .props
            .class
            .clone();
        class.push(DROPDOWN_CLASS);
        class.push(self.props.placement.map(|c| c.class(DROPDOWN_CLASS)));

        const BTN_CLASS: &str = "mui-btn";
        let mut button_class = Classes::from(BTN_CLASS);
        button_class.push(self.props.color.map(|c| c.class(BTN_CLASS)));
        button_class.push(self.props.size.map(|c| c.class(BTN_CLASS)));
        button_class.push(self.props.variant.map(|c| c.class(BTN_CLASS)));

        const MENU_CLASS: &str = "mui-dropdown__menu";
        let mut ul_class = Classes::from(MENU_CLASS);
        ul_class.push(self.props.alignment.map(|c| c.class(MENU_CLASS)));

        html! {
            <div class={class}>
                <button class={button_class}
                    disabled={self.props.disabled}
                    data-mui-toggle="dropdown">
                    { &self.props.label }
                    <Caret direction={self.props.placement.map(Placement::direction)} />
                </button>
                <ul class={ul_class}>
                    { self.props.children.clone() }
                </ul>
            </div>
        }
    }
}
