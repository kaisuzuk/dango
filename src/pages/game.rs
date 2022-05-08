use yew::prelude::*;

pub struct Game;

impl Component for Game {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1 class="title">
                { "Game" }
            </h1>
        }
    }
}
