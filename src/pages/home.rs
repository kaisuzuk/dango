use yew::prelude::*;
use yew_router::prelude::*;
use yew::html::Scope;
use yew_router::components::Link;
use crate::Route;

pub struct HomeModel;

pub enum Msg {
    Submit,
}

impl Component for HomeModel {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
                log::info!("submit");
                log::info!("{:?}", Route::NotFound);
                // validInput? 
                // yes  -> submitして画面遷移(Game)
                // no   -> alert出して画面遷移しない
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            { self.view_title() }
            { self.view_form(ctx.link()) }
            <Link<Route> to={Route::NotFound}>{ "click here to go home" }</Link<Route>>
            </>
        }
    }
}
impl HomeModel {
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

                <button onclick={link.callback(|_| Msg::Submit)} class="mt-4 btn btn-primary">{"Submit"}</button>
            </div>
        }
    }
}
