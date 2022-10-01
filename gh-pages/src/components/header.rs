use crate::switch::{AppAnchor, AppRoute};
use muicss_yew::{appbar::Appbar, container::Container};
use yew::prelude::*;
use yew_feather::github::Github;

#[derive(Clone, Debug)]
pub struct Header;

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: &Scope<Self>) -> Self {
        Header
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <header>
                <Appbar>
                    <Container fluid=true>
                        <table width="100%">
                            <tbody>
                                <tr class="mui--appbar-height">
                                    <td class="mui--text-headline">
                                        <AppAnchor route=AppRoute::Home>
                                            <span id="title" >
                                                { "MUICSS-Yew" }
                                            </span>
                                        </AppAnchor>
                                    </td>
                                    <td class="mui--text-right">
                                        <a href="https://github.com/AlephAlpha/muicss-yew">
                                            <Github color="#FFF" />
                                        </a>
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </Container>
                </Appbar>
            </header>
        }
    }
}
