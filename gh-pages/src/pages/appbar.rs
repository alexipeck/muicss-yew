use crate::components::example::Example;
use muicss_yew::appbar::Appbar;
use yew::prelude::*;
use yew_prism::Prism;

#[derive(Clone, Debug)]
pub struct AppbarExamples;

impl Component for AppbarExamples {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: &Scope<Self>) -> Self {
        AppbarExamples
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
            </>
        }
    }
}

impl AppbarExamples {
    const IMPORT: &'static str = r#"use muicss_yew::appbar::Appbar;"#;

    fn introduction(&self) -> Html {
        html! {
            <>
                <h1>
                    { "Appbar" }
                </h1>
                <Prism code=Self::IMPORT language="rust" />
                <p>
                    { "See " }
                    <a href="https://www.muicss.com/docs/v1/css-js/appbar">
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
                    { "Container Element" }
                </h2>
                <Example code=include_str!("../examples/appbar_example1.rs")>
                    { include!("../examples/appbar_example1.rs") }
                </Example>
            </>
        }
    }

    fn example2(&self) -> Html {
        html! {
            <>
                <h2>
                    { "Helpers" }
                </h2>
                <Example code=include_str!("../examples/appbar_example2.rs")>
                    { include!("../examples/appbar_example2.rs") }
                </Example>
            </>
        }
    }
}
