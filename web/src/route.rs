mod index;
mod not_found;

use yew::prelude::*;
use yew_router::prelude::*;

pub type Link = yew_router::components::Link<Route>;
pub type Redirect = yew_router::components::Redirect<Route>;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/echo/:s")]
    Echo { s: String },

    #[at("/about")]
    About,

    #[at("/")]
    Index,

    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(About)]
fn about() -> Html {
    html! {
        <section class="section is-large">
            <h1 class="title">
                { "Here create a goooooood page about yourself/company" }
            </h1>
        </section>
    }
}

impl Route {
    fn name(&self) -> &'static str {
        match self {
            Self::Index => "Yew App",
            Self::NotFound => "404 NotFound",
            Self::About => "About Me",
            Self::Echo { .. } => "Echo",
        }
    }
    
    pub fn render(&self) -> Html {
        crate::utils::change_title(self.name());
        
        match self {
            Self::Echo { s } => html! {
                <section class="section is-large">
                    <h1 class="title">{ "Echoing" }</h1>
                    <h2 class="subtitle">{ s }</h2>
                </section>
            },

            Self::Index => html! {
                <index::Index/>
            },

            Self::About => html! {
                <About/>
            },

            Self::NotFound => html! {
                <not_found::NotFound/>
            },
        }
    }
}
