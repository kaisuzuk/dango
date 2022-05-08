// use yew::prelude::*;
// use yew::classes;
// use std::collections::HashMap;
// use yew_router::prelude::*;
// // mod app;

// enum Msg {
//     SubmitSetting,
//     GameStart,
//     NextTurn,
//     Result,
// }

// enum Dango {
//     Nomal = 0,
//     Gezai = 1,
//     Dead = 2,
// }

// enum Status {
//     Nomal = 0,
//     Gezai = 1,
// }

// enum PlayerID {
//     Player1 = 0,
//     Player2 = 1,
// }

// pub struct Model {
//     cells: Vec<Dango>,
//     able_to_eat_cnt: u32,
//     whose_turn: PlayerID,
//     players: HashMap<PlayerID, Player>,
// }

// struct Player {
//     status: Status,
//     api_file_name: String,
// }

// impl Component for Model {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self {
//             cells: vec![],
//             able_to_eat_cnt: 2,
//             whose_turn: PlayerID::Player1,
//             players: HashMap::default(),
//         }
//     }

//     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::SubmitSetting => {
//                 return true;
//             },
//             Msg::GameStart => {
//                 return true;
//             },
//             Msg::NextTurn => {
//                 return true;
//             },
//             Msg::Result => {
//                 return true;
//             }
//         }
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
//         let link = ctx.link();
//         html! {
//             <div>
//                 <Switch<Route> render={switch}/>
//                 // <div>
//                 //     <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
//                 //     <p>{ self.value }</p>
//                 // </div>
//                 // <div class={classes!("dango-area")}>
//                 //     <div class={classes!("dango")}></div>
//                 //     <div class={classes!("dango")}></div>
//                 //     <div class={classes!("dango")}></div>
//                 //     <div class={classes!("dango")}></div>
//                 //     <div class={classes!("dango")}></div>
//                 // </div>
//             </div>
//         }
//     }
// }

// #[function_component(Secure)]
// fn secure() -> Html {
//     let history = use_history().unwrap();

//     let onclick = Callback::once(move |_| history.push(Route::Home));
//     html! {
//         <div>
//             <h1>{ "Secure" }</h1>
//             <button {onclick}>{ "Go Home" }</button>
//         </div>
//     }
// }
// // fn switch(routes: &Route) -> Html {
// //     match routes {
// //         Route::Home => html! { <h1>{ "Home" }</h1> },
// //         Route::Secure => html! {
// //             <Secure />
// //         },
// //         Route::NotFound => html! { <h1>{ "404" }</h1> },
// //     }
// // }

// // #[function_component(Main)]
// // pub fn App() -> Html {
// //     html! {
// //         <BrowserRouter>
// //             <Switch<Route> render={Switch::render(switch)} />
// //         </BrowserRouter>
// //     }
// // }
