use crate::components::example::Example;
use muicss_yew::divider::Divider;
use yew::prelude::*;
use yew_prism::Prism;

#[derive(Clone, Debug)]
pub struct DividerExamples;

impl Component for DividerExamples {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: &Scope<Self>) -> Self {
        DividerExamples
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

impl DividerExamples {
    const IMPORT: &'static str = r#"use muicss_yew::divider::Divider;"#;

    fn introduction(&self) -> Html {
        html! {
            <>
                <h1>
                    { "Divider" }
                </h1>
                <Prism code=Self::IMPORT language="rust" />
                <p>
                    { "See " }
                    <a href="https://www.muicss.com/docs/v1/css-js/dividers">
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
                <Example code=include_str!("../examples/divider_example1.rs")>
                    { include!("../examples/divider_example1.rs") }
                </Example>
            </>
        }
    }
}
