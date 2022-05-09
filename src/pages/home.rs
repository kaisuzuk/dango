use yew::prelude::*;
use yew_router::prelude::*;
use yew::html::Scope;
use yew_router::components::Link;
use crate::Route;

pub struct Home;

pub enum Msg {
    Save,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Save => {
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            { self.view_title() }
            { self.view_form(ctx.link()) }
            <Link<Route> to={Route::Game}>{ "click here to play dango game!" }</Link<Route>>
            </>
        }
    }
}
impl Home {
    fn view_title(&self) -> Html {
        html!{
            <div class="container px-4 py-2">
                <h1 class="title">{"だんごゲーム"}</h1>
            </div>
        }
    }
    fn view_form(&self, link: &Scope<Self>) -> Html {
        html! {
            <div class="container form border">
                <div class="mb-3 mx-400">
                    <label class="form-label">{"だんごの数"}</label>
                    <input type="number" class="form-control" min=1 value=30/>
                </div>

                <div class="mb-3 mx-400">
                    <label class="form-label">{"下剤の数"}</label>
                    <input type="number" class="form-control" min=0 value=1/>
                </div>

                <div class="mb-3 mx-400">
                    <label class="form-label">{"一度に食べられる数"}</label>
                    <input type="number" class="form-control" min=1 value=3/>
                </div>

                <div class="mb-3 mx-400">
                    <label class="form-label">{"1P API"}</label>
                    <input class="form-control" type="file" />
                </div>

                <div class="mb-3 mx-400">
                    <label class="form-label">{"2P API"}</label>
                    <input class="form-control" type="file" />
                </div>

                <button onclick={link.callback(|_| Msg::Save)} class="mt-4 btn btn-primary">{"入力内容を保存"}</button>
            </div>
        }
    }
}