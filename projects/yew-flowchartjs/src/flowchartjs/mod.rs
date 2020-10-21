pub use flowchartjs_wasmbind::{draw_flow_chart};
use yew::{prelude::*, Component, ComponentLink, Html, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct FlowChartJSProperties {
    pub code: String,
}

pub struct FlowChartJS {
    pub props: FlowChartJSProperties,
}

impl Component for FlowChartJS {
    type Message = ();
    type Properties = FlowChartJSProperties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        match self.props == props {
            true => false,
            false => {
                self.props = props;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let t = yew::utils::document().create_element("span").unwrap();
        t.set_inner_html(&draw_flow_chart(&self.props.code));
        Html::VRef(t.into())
    }
}
