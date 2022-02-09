mod route;
pub mod utils;

use route::{Link, Route};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

enum Msg {
    ToggleNavbar
}

struct App {
    show_navbar: bool,
}

impl App {
    fn view_nav(&self, ctx: &Context<Self>) -> Html {
        let active_class = if self.show_navbar {
            "is-active"
        } else {
            ""
        };
        
        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "Yew App" }</h1>

                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={ctx.link().callback(|_| Msg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link classes={classes!("navbar-item")} to={Route::Index}>
                            { "Home" }
                        </Link>

                        <div class="navbar-item has-dropdown is-hoverable">
                            <div class="navbar-link">
                                { "More" }
                            </div>
                            <div class="navbar-dropdown">
                                <Link classes={classes!("navbar-item")} to={Route::About}>
                                    { "About me" }
                                </Link>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>
        }
    }
    
    fn view_footer(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <footer class="footer">
                <div class="content has-text-centered">
                    { "Powered by " }
                    <a href="https://yew.rs">{ "Yew" }</a>
                    { " using " }
                    <a href="https://bulma.io">{ "Bulma" }</a>
                    { " and images from " }
                    <a href="https://unsplash.com">{ "Unsplash" }</a>
                </div>
            </footer>
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        
        Self { show_navbar: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => self.show_navbar = !self.show_navbar,
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx) }
                <Switch<Route> render={Switch::render(Route::render)} />
                { self.view_footer(ctx) }
            </BrowserRouter>
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    wasm_logger::init(Default::default());
    yew::start_app::<App>();
}
