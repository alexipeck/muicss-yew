use crate::components::example::Example;
use muicss_yew::button::{Button, Color, Size, Variant};
use yew::prelude::*;
use yew_prism::Prism;

#[derive(Clone, Debug)]
pub struct ButtonExamples;

impl Component for ButtonExamples {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        ButtonExamples
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
                { self.example5() }
            </>
        }
    }
}

impl ButtonExamples {
    const IMPORT: &'static str = r#"use muicss_yew::button::{Button, Color, Size, Variant};"#;

    fn introduction(&self) -> Html {
        html! {
            <>
                <h1>
                    { "Buttons" }
                </h1>
                <Prism code={Self::IMPORT} language="rust" />
                <p>
                    { "See " }
                    <a href="https://www.muicss.com/docs/v1/css-js/buttons">
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
                    { "Default Buttons" }
                </h2>
                <Example code={include_str!("../examples/buttons_example1.rs")}>
                    { include!("../examples/buttons_example1.rs") }
                </Example>
            </>
        }
    }

    fn example2(&self) -> Html {
        html! {
            <>
                <h2>
                    { "Flat Buttons" }
                </h2>
                <Example code={include_str!("../examples/buttons_example2.rs")}>
                    { include!("../examples/buttons_example2.rs") }
                </Example>
            </>
        }
    }

    fn example3(&self) -> Html {
        html! {
            <>
                <h2>
                    { "Raised Buttons" }
                </h2>
                <Example code={include_str!("../examples/buttons_example3.rs")}>
                    { include!("../examples/buttons_example3.rs") }
                </Example>
            </>
        }
    }

    fn example4(&self) -> Html {
        html! {
            <>
                <h2>
                    { "Floating Action Buttons" }
                </h2>
                <Example code={include_str!("../examples/buttons_example4.rs")}>
                    { include!("../examples/buttons_example4.rs") }
                </Example>
            </>
        }
    }

    fn example5(&self) -> Html {
        html! {
            <>
                <h2>
                    { "Buttons sizes" }
                </h2>
                <Example code={include_str!("../examples/buttons_example5.rs")}>
                    { include!("../examples/buttons_example5.rs") }
                </Example>
            </>
        }
    }
}
