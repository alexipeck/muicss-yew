use crate::switch::{AppAnchor, AppRoute};
use yew::prelude::*;
use yew_prism::Prism;

#[derive(Clone, Debug)]
pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Home
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
                { self.usage() }
                { self.examples() }
            </>
        }
    }
}

impl Home {
    const STATIC_HTML: &'static str = r#"<link rel="stylesheet" href="https://cdn.muicss.com/mui-0.10.3/css/mui.min.css" />
<script src="https://cdn.muicss.com/mui-0.10.3/js/mui.min.js"></script>"#;
    const CARGO_TOML: &'static str = r#"[dependencies]
muicss-yew = { git = "https://github.com/AlephAlpha/muicss-yew" }"#;

    fn introduction(&self) -> Html {
        html! {
            <>
                <h1>
                    { "Introduction" }
                </h1>
                <p>
                    { "MUICSS-Yew is a component library for " }
                    <a href="https://yew.rs">
                        {"Yew"}
                    </a>
                    { " framework based on the lightweight " }
                    <a href="https://www.muicss.com">
                        {"MUI"}
                    </a>
                    { " CSS framework." }
                </p>
                <p>
                    { "This project is still work in progress. \n\
                       Many components are not yet supported. \n\
                       If those components are needed, \n\
                       you may use MUI-CSS directly (i.e., " }
                    <a href="https://www.muicss.com/docs/v1/css-js/boilerplate-html">
                        {"with CSS and JS"}
                    </a>
                    {") with Yew." }
                </p>
                <p>
                    { "For other Yew component libraries, please see " }
                    <a href="https://github.com/jetli/awesome-yew#component-libraries">
                        {"awesome-yew"}
                    </a>
                    {"." }
                </p>
            </>
        }
    }

    fn usage(&self) -> Html {
        html! {
            <>
                <h2>
                    { "Usage" }
                </h2>
                <p>
                    { "Add these two lines to your static html file:" }
                </p>
                <Prism code={Self::STATIC_HTML} language="html" />
                <p>
                    { "Add muicss-yew to your Cargo.toml:" }
                </p>
                <Prism code={Self::CARGO_TOML} language="toml" />
                <p>
                    { "For details, please see the examples." }
                </p>
            </>
        }
    }

    fn examples(&self) -> Html {
        html! {
            <>
                <h2>
                    { "Examples" }
                </h2>
                <ul>
                    <li>
                        <AppAnchor route={AppRoute::Appbar}>
                            { "Appbar" }
                        </AppAnchor>
                    </li>
                    <li>
                        <AppAnchor route={AppRoute::Buttons}>
                            { "Buttons" }
                        </AppAnchor>
                    </li>
                    <li>
                        <AppAnchor route={AppRoute::Caret}>
                            { "Caret" }
                        </AppAnchor>
                    </li>
                    <li>
                        <AppAnchor route={AppRoute::Checkbox}>
                            { "Checkbox" }
                        </AppAnchor>
                    </li>
                    <li>
                        <AppAnchor route={AppRoute::Container}>
                            { "Container" }
                        </AppAnchor>
                    </li>
                    <li>
                        <AppAnchor route={AppRoute::Divider}>
                            { "Divider" }
                        </AppAnchor>
                    </li>
                    <li>
                        <AppAnchor route={AppRoute::Dropdowns}>
                            { "Dropdowns" }
                        </AppAnchor>
                    </li>
                    <li>
                        <AppAnchor route={AppRoute::Grid}>
                            { "Grid" }
                        </AppAnchor>
                    </li>
                    <li>
                        <AppAnchor route={AppRoute::Input}>
                            { "Input" }
                        </AppAnchor>
                    </li>
                    <li>
                        <AppAnchor route={AppRoute::Panels}>
                            { "Panels" }
                        </AppAnchor>
                    </li>
                    <li>
                        <AppAnchor route={AppRoute::Textarea}>
                            { "Textarea" }
                        </AppAnchor>
                    </li>
                </ul>
            </>
        }
    }
}
