use yew::prelude::*;
use yew_prism::Prism;

#[derive(Clone, Debug)]
pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Home
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
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

                <h2>
                    { "Usage" }
                </h2>
                <p>
                    { "Add these two lines to your static html file:" }
                </p>
                <Prism
                    code="<link rel=\"stylesheet\" href=\"https://cdn.muicss.com/mui-0.10.3/css/mui.min.css\" />\n\
                          <script src=\"https://cdn.muicss.com/mui-0.10.3/js/mui.min.js\"></script>"
                    language="html"
                />
                <p>
                    { "Add muicss-yew to your Cargo.toml:" }
                </p>
                <Prism
                    code="[dependencies]\n\
                    muicss-yew = { git = \"https://github.com/AlephAlpha/muicss-yew\" }"
                    language="toml"
                />
                <p>
                    { "For details, please see the " }
                    <a href="https://github.com/AlephAlpha/muicss-yew/tree/master/examples">
                        {"examples"}
                    </a>
                    {"." }
                </p>
            </>
        }
    }
}