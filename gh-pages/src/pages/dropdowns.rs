use crate::components::example::Example;
use muicss_yew::{
    button::Color,
    dropdown::{Alignment, Dropdown, Placement},
    dropdown_item::DropdownItem,
};
use yew::prelude::*;
use yew_prism::Prism;

#[derive(Clone, Debug)]
pub struct DropdownExamples;

impl Component for DropdownExamples {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        DropdownExamples
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { self.introduction() }
                { self.example1() }
                { self.example2() }
                { self.example3() }
                { self.example4() }
            </>
        }
    }
}

impl DropdownExamples {
    const IMPORT: &'static str = r#"use muicss_yew::button::Color;
use muicss_yew::dropdown::{Alignment, Dropdown, Placement};
use muicss_yew::dropdown_item::DropdownItem;"#;

    fn introduction(&self) -> Html {
        html! {
            <>
                <h1>
                    { "Dropdowns" }
                </h1>
                <Prism code={Self::IMPORT} language="rust" />
                <p>
                    { "See " }
                    <a href="https://www.muicss.com/docs/v1/css-js/dropdowns">
                        { "MUI's website" }
                    </a>
                    { " for details." }
                </p>
            </>
        }
    }

    fn example1(&self) -> Html {
        html! {
            <>
                <h2>
                    { "Dropdown (default)" }
                </h2>
                <Example code={include_str!("../examples/dropdowns_example1.rs")}>
                    { include!("../examples/dropdowns_example1.rs") }
                </Example>
            </>
        }
    }

    fn example2(&self) -> Html {
        html! {
            <>
                <h2>
                    { "Dropup" }
                </h2>
                <Example code={include_str!("../examples/dropdowns_example2.rs")}>
                    { include!("../examples/dropdowns_example2.rs") }
                </Example>
            </>
        }
    }

    fn example3(&self) -> Html {
        html! {
            <>
                <h2>
                    { "Dropright" }
                </h2>
                <Example code={include_str!("../examples/dropdowns_example3.rs")}>
                    { include!("../examples/dropdowns_example3.rs") }
                </Example>
            </>
        }
    }

    fn example4(&self) -> Html {
        html! {
            <>
                <h2>
                    { "Dropleft" }
                </h2>
                <Example code={include_str!("../examples/dropdowns_example4.rs")}>
                    { include!("../examples/dropdowns_example4.rs") }
                </Example>
            </>
        }
    }
}
