use crate::components::example::Example;
use muicss_yew::panel::Panel;
use yew::prelude::*;
use yew_prism::Prism;

#[derive(Clone, Debug)]
pub struct PanelExamples;

impl Component for PanelExamples {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        PanelExamples
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

impl PanelExamples {
    const IMPORT: &'static str = r#"use muicss_yew::panel::Panel;"#;

    fn introduction(&self) -> Html {
        html! {
            <>
                <h1>
                    { "Panels" }
                </h1>
                <Prism code={Self::IMPORT} language="rust" />
                <p>
                    { "See " }
                    <a href="https://www.muicss.com/docs/v1/react/panels">
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
                <Example code={include_str!("../examples/panel_example1.rs")}>
                    { include!("../examples/panel_example1.rs") }
                </Example>
            </>
        }
    }
}
