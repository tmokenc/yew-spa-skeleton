mod index;

use yew::prelude::*;
use yew_router::prelude::*;

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

impl Route {
    pub fn render(&self) -> Html {
        match self {
            Self::Echo { s } => html! {
                <>
                    <h1>{ "Echoing" }</h1>
                    <h2>{ s }</h2>
                </>
            },

            Self::Index => html! {
                <h1>{ "Hi, I'm Tomoka" }</h1>
            },

            Self::About => html! {
                <h1>{ "By tmokenc" }</h1>
            },

            Self::NotFound => html! {
                <h1>{ "404 Successfully NOT FOUND" }</h1>
            },
        }
    }
}
