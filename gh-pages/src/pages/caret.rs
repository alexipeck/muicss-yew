use crate::components::example::Example;
use muicss_yew::caret::{Caret, Direction};
use yew::prelude::*;
use yew_prism::Prism;

#[derive(Clone, Debug)]
pub struct CaretExamples;

impl Component for CaretExamples {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        CaretExamples
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
            </>
        }
    }
}

impl CaretExamples {
    const IMPORT: &'static str = r#"use muicss_yew::caret::{Caret, Direction};"#;

    fn introduction(&self) -> Html {
        html! {
            <>
                <h1>
                    { "Caret" }
                </h1>
                <Prism code={Self::IMPORT} language="rust" />
                <p>
                    { "MUI's website doesn't have a page for this component." }
                </p>
            </>
        }
    }

    fn example1(&self) -> Html {
        html! {
            <>
                <Example code={include_str!("../examples/caret_example1.rs")}>
                    { include!("../examples/caret_example1.rs") }
                </Example>
            </>
        }
    }
}
