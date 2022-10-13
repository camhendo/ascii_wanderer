
use yew::{events::{KeyboardEvent, MouseEvent}, html::{self, IntoPropValue}, Component, Context, Html, NodeRef};
use web_sys::{Element, Node, HtmlElement, Document};
// use wasm_bindgen::JsValue;
use gloo_utils::document;
use yew::prelude::*;
use yew_utils::components::table::Table;
use yew_utils::vdom::*;
use slab::Slab;

mod gamearea;
use gamearea::{PlayAreaApp, PlayAreaProps};

struct AppModel {
    apps: Slab<(Element, AppHandle<PlayAreaApp>)>,
    apps_container_ref: NodeRef,
    game_underway: bool,
    game_button_text: String,
}

enum AppMsg {
    SpawnGameInstance, 
    ResetGameInstance(usize),
}

impl Component for AppModel {
    type Message = AppMsg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            apps: Slab::new(),
            apps_container_ref: NodeRef::default(),
            game_underway: false,
            game_button_text: String::from("Start Game"),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {

        let app_container = self.apps_container_ref
            .cast::<Element>()
            .expect("Failed to cast app container div to HtmlElement");

        match msg {
            AppMsg::SpawnGameInstance => {

                self.game_button_text = String::from("Continue Game");
                if self.game_underway {
                    if let Some( elem) = self.apps[0].1.get_component() {
                        elem.text_input.cast::<HtmlElement>().unwrap().focus().unwrap();
                    }
                    
                } else {
                    let app_div = document()
                        .create_element("div")
                        .expect("Failed to create <div> element");
                    
                    //add styling class
                    app_div.set_class_name("none");
                    
                    let _ = app_container
                        .append_child(&app_div)
                        .expect("Failed to append app div app container div");

                    let app_entry = self.apps.vacant_entry();

                    // TODO: use app_key for reset of the game state.
                    let app_key = app_entry.key();


                    let new_game_app = yew::start_app_with_props_in_element(
                        app_div.clone(),
                        PlayAreaProps {
                            player_loc: NodeRef::default(),
                            reset_callback: ctx.link().callback(move |_| AppMsg::ResetGameInstance(app_key))
                        },
                    );

                    app_entry.insert((app_div, new_game_app));

                    
                    self.game_underway = true;

                    document().get_element_by_id("start-game").unwrap().set_inner_html("Continue Game");
                }
            }
            AppMsg::ResetGameInstance(app_id) => {
                let (app_div, _app) = self.apps.remove(app_id);

                app_div.remove();

                self.game_underway = false;
                document().get_element_by_id("start-game").unwrap().set_inner_html("Start Game");
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        html! {
            <>
                <h1 id="main-title">{"Ascii Wanderer"}</h1>
                <div class="main-container" ref={self.apps_container_ref.clone()}>
                    <button id="start-game" onclick={ctx.link().callback( |_| AppMsg::SpawnGameInstance)}></button>
                </div>
            </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }
        document().get_element_by_id("start-game").unwrap().set_inner_html("Start Game");

    }
}


fn main() {
    yew::start_app::<AppModel>();
}

