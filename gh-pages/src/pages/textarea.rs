use crate::components::example::Example;
use muicss_yew::textarea::Textarea;
use yew::prelude::*;
use yew_prism::Prism;

#[derive(Clone, Debug)]
pub struct TextareaExamples;

impl Component for TextareaExamples {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        TextareaExamples
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
            </>
        }
    }
}

impl TextareaExamples {
    const IMPORT: &'static str = r#"use muicss_yew::textarea::Textarea;"#;

    fn introduction(&self) -> Html {
        html! {
            <>
                <h1>
                    { "Textarea" }
                </h1>
                <Prism code={Self::IMPORT} language="rust" />
                <p>
                    { "See " }
                    <a href="https://www.muicss.com/docs/v1/react/textarea">
                        { "MUI's website" }
                    </a>
                    { " for details. But this Yew component does not support so many \
                       properties as the React component." }
                </p>
            </>
        }
    }

    fn example1(&self) -> Html {
        html! {
            <>
                <h2>
                    { "Basic example" }
                </h2>
                <Example code={include_str!("../examples/textarea_example1.rs")}>
                    { include!("../examples/textarea_example1.rs") }
                </Example>
            </>
        }
    }

    fn example2(&self) -> Html {
        html! {
            <>
                <h2>
                    { "Fixed labels" }
                </h2>
                <Example code={include_str!("../examples/textarea_example2.rs")}>
                    { include!("../examples/textarea_example2.rs") }
                </Example>
            </>
        }
    }

    fn example3(&self) -> Html {
        html! {
            <>
                <h2>
                    { "Floating labels" }
                </h2>
                <Example code={include_str!("../examples/textarea_example3.rs")}>
                    { include!("../examples/textarea_example3.rs") }
                </Example>
            </>
        }
    }
}
