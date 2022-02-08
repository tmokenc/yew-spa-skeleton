mod route;

use route::Route;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

enum Msg {
    ShowNavbar,
    HideNavbar,
}

struct App {
    show_navbar: bool,
}

impl App {
    fn view_nav(&self, ctx: &Context<Self>) -> Html {
        if self.show_navbar {
            html! {
                <div></div>
            }
        } else {
            html! {
                <nav>{"nav"}</nav>
            }
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { show_navbar: false }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowNavbar => self.show_navbar = true,
            Msg::HideNavbar => self.show_navbar = false,
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                { self.view_nav(ctx) }

                <BrowserRouter>
                    <Switch<Route> render={Switch::render(Route::render)} />
                </BrowserRouter>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    wasm_logger::init(Default::default());
    yew::start_app::<App>();
}
