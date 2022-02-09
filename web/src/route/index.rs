use yew::prelude::*;
use super::{Link, Route};

pub struct Index;

impl Component for Index {
    type Message = ();
    type Properties = ();
    
    fn create(_: &Context<Self>) -> Self {
        Self
    }
    
    fn view(&self, _: &Context<Self>) -> Html {
        html!{
            <>
            <section id="banner" class="hero is-white is-fullheight">
                <div class="hero-body">
                    <div class="container">
                        <div class="columns is-vcentered reverse-columns">
                            <div class="column is-10-mobile is-10-tablet is-5-desktop is-5-widescreen is-5-fullhd">
                                <h1 class="title has-text-white is-1 mb-6">
                                    { "Yew SPA Skeleton" }
                                </h1>
                                <h2 class="subtitle has-text-white">
                                    { "This is just a Yew SPA template for you to start a Yew project" }
                                </h2>
                                
                                <div class="buttons">
                                    <button class="button is-yellow">
                                        <a href="https://github.com/tmokenc/yew-spa-skeleton" target="_blank" rel="noopener">
                                            { "Source code" }
                                        </a>
                                    </button>
                                    <button class="button">
                                        <Link to={Route::About}>
                                            { "About me" }
                                        </Link>
                                    </button>
                                </div>
                            </div>
                        
                            <div class="column is-10-mobile is-10-tablet is-4-desktop is-7-widescreen is-4-fullhd is-offset-1-fullhd">
                                <figure class="image is-square">
                                    <img src="/static/img/coffe.jpg"/>
                                </figure>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            
            <section class="hero is-medium has-text-centered">
                <div class="hero-body">
                    <div class="container">
                        <h1 class="title is-1 mb-6">
                            { "Written entirely in Rust" }
                        </h1>
                        <h2 class="subtitle">
                            { "Fast & Reliability" }
                        </h2>
                    </div>
                </div>
            </section>
            </>
        }
    }
}
