use yew::prelude::*;

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                { self.view_div() }
            </div>
        }
    }
}
impl Home {
    fn view_div(&self) -> Html {
        html! {
            <>
            {"Hello!"}
            </>
        }
    }
}
