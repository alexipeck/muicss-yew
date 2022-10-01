use muicss_yew::{col::Col, container::Container, row::Row};
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, Debug)]
pub struct Main {
    props: Props,
}

impl Component for Main {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Main {
            props: ctx.props().to_owned(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props.neq_assign(ctx.props().to_owned())
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <main>
                <Container fluid=true>
                    <Row>
                        <Col sm=10 sm_offset=1>
                            { self.props.children.clone() }
                        </Col>
                    </Row>
                </Container>
            </main>
        }
    }
}
