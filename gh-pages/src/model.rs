use crate::{
    components::{header::Header, main::Main},
    pages::{
        appbar::AppbarExamples, buttons::ButtonExamples, caret::CaretExamples,
        checkbox::CheckboxExamples, container::ContainerExamples, divider::DividerExamples,
        dropdowns::DropdownExamples, grid::GridExamples, home::Home, input::InputExamples,
        panels::PanelExamples, textarea::TextareaExamples,
    },
    switch::{AppRoute, AppRouter, PublicUrlSwitch},
};
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: &Scope<Self>) -> Self {
        Model
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
                <Header />
                <Main>
                    <AppRouter render=Router::render(Self::switch) />
                </Main>
            </>
        }
    }
}

impl Model {
    fn switch(switch: PublicUrlSwitch) -> Html {
        match switch.route() {
            AppRoute::Appbar => html! { <AppbarExamples /> },
            AppRoute::Buttons => html! { <ButtonExamples /> },
            AppRoute::Caret => html! { <CaretExamples /> },
            AppRoute::Checkbox => html! { <CheckboxExamples /> },
            AppRoute::Container => html! { <ContainerExamples /> },
            AppRoute::Divider => html! { <DividerExamples /> },
            AppRoute::Dropdowns => html! { <DropdownExamples /> },
            AppRoute::Grid => html! { <GridExamples /> },
            AppRoute::Input => html! { <InputExamples /> },
            AppRoute::Panels => html! { <PanelExamples /> },
            AppRoute::Textarea => html! { <TextareaExamples /> },
            AppRoute::Home => html! { <Home /> },
        }
    }
}
