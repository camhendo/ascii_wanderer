
use std::fmt::Debug;

use gloo_events::EventListener;
use yew::{events::{KeyboardEvent, MouseEvent}, html::{self, IntoPropValue}, Component, Context, Html, NodeRef};
use web_sys::{Element, Node, HtmlElement, Document, CustomEvent};
// use wasm_bindgen::JsValue;
use gloo_utils::document;
use gloo::console;
use yew::prelude::*;
use yew_utils::components::table::Table;
use yew_utils::vdom::*;
use slab::Slab;


mod arena;
use arena::Arena;

mod popups;
use popups::{PopUp, PopUpProps};

pub enum Msg {
    Move(String),
    PopUp(String),
    DestroyPopUp(usize)
}

pub struct PlayAreaApp {
    x : i32,
    y : i32,
    lox: i32,
    loy: i32,
    pub td_width: f64,
    pub td_height: f64,
    player_loc: (f64, f64,f64, f64),
    pub text_input: NodeRef,
    play_area: NodeRef,
    arena: Vec<String>,
    popup_ref: NodeRef,
    popup: Slab<(Element, AppHandle<PopUp>)>,
    popped: bool,
    last_popup_key: usize,
    pub is_hover: bool,
    pub hover: String,
}

#[derive(Clone, Properties, PartialEq)]
pub struct PlayAreaProps {
    pub player_loc: NodeRef,
    pub reset_callback: Callback<()>
}

impl Component for PlayAreaApp {

    type Message = Msg;
    type Properties = PlayAreaProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            x: 50,
            y: 25,
            lox: 0,
            loy: 0,
            td_width: 0.0,
            td_height: 0.0,
            player_loc: (0.0,0.0,0.0,0.0),
            text_input: NodeRef::default(),
            play_area: NodeRef::default(),
            arena: Arena::new(50, 100),
            popup_ref: NodeRef::default(),
            popup: Slab::new(),
            popped: false,
            last_popup_key: 0,
            is_hover: false,
            hover: "Nothing".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        
        let app_container = self.popup_ref
                .cast::<Element>()
                .expect("Failed to cast app container div to HtmlElement");

        let finder = self.arena.clone();

        let bounded_play_area = document().get_element_by_id("playarea").unwrap().get_bounding_client_rect();

        let td = document().get_element_by_id("current-location").unwrap().get_bounding_client_rect();

        match msg {
            Msg::Move(code) => {

                if self.popped {
                    let destroy = ctx.link().callback(Msg::DestroyPopUp);
                    destroy.emit(self.last_popup_key.clone());
                };

                
                if code == "KeyW" {
                    self.y -= 1;
                    self.loy -= self.td_height.clone() as i32;
                    self.player_loc = (td.x(), td.y() - self.td_height.clone(), self.td_width.clone(), self.td_height.clone());
                } 
                else if code == "KeyS" {
                    self.y += 1;
                    self.loy += self.td_height.clone() as i32;
                    self.player_loc = (td.x(), td.y() + self.td_height.clone(), self.td_width.clone(), self.td_height.clone());
                    
                } 
                else if code == "KeyA" {
                    self.x -= 1;
                    self.lox -= self.td_width.clone() as i32;
                    self.player_loc = (td.x() - self.td_width.clone(), td.y().clone(), self.td_width.clone(), self.td_height.clone());
                    
                } 
                else if code == "KeyD" {
                    self.x += 1;
                    self.lox += self.td_width.clone() as i32;
                    self.player_loc = (td.x() + self.td_width.clone(), td.y().clone(), self.td_width.clone(), self.td_height.clone());

                };

                
                let pa = document().get_element_by_id("playarea").unwrap();
                if td.x().clone() < bounded_play_area.left().clone() || (td.x().clone()+self.td_width.clone()) > bounded_play_area.right().clone() {
                    pa.set_scroll_left(self.lox);
                }
                if td.y().clone() < bounded_play_area.top().clone() || (td.y().clone()+self.td_height.clone()) > bounded_play_area.bottom().clone() {
                    pa.set_scroll_top(self.loy);
                }
                
                let z = finder.clone().get((self.x.clone()+(100*self.y.clone())) as usize).unwrap().to_string();
                if  z != ".  " && code != "Edge"{
                    let nft = ctx.link().callback_once(Msg::PopUp);
                    nft.emit(z.to_owned());
                }
                
                true
            }
            Msg::PopUp(var) => {

                console::log!("You found one!");
                
                if self.popped {
                    let destroy = ctx.link().callback(Msg::DestroyPopUp);
                    destroy.emit(self.last_popup_key.clone());
                };

                let app_div = document()
                        .create_element("div")
                        .expect("Failed to create <div> element");
                    
                
                let _ = app_container
                    .append_child(&app_div)
                    .expect("Failed to append app div app container div");

                let app_entry = self.popup.vacant_entry();

                let app_key = app_entry.key();

                let new_game_app = yew::start_app_with_props_in_element(
                    app_div.clone(),
                    PopUpProps {
                        thing: var,
                        location: self.player_loc.clone(),
                        bounding_rect: bounded_play_area,
                        destroy_popup: ctx.link().callback(move |_| Msg::DestroyPopUp(app_key))
                    },
                );
                
                self.last_popup_key = app_key;
                self.popped = true;
                app_entry.insert((app_div, new_game_app));

                true
            }
            Msg::DestroyPopUp(app_id) => {
                let (app_div, app) = self.popup.remove(app_id);
                app.destroy();
                app_div.remove();
                self.popped = false;
                if let Some(elem) = self.text_input.cast::<HtmlElement>() {
                    elem.focus().unwrap();
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        const FIELD_X: i32 = 100;
        const FIELD_Y: i32 = 50;
        let pos_x = self.x.clone();
        let pos_y = self.y.clone();

        
        let onkeydown = ctx.link().batch_callback(move |event: KeyboardEvent| {
            let poy = pos_y.clone();
            let pox = pos_x.clone();
            let code = event.code();

            if (code == "KeyW".to_string() && poy > 0) 
                || (code == "KeyS" && poy < FIELD_Y-1) 
                || (code == "KeyA" && pox > 0)
                || (code == "KeyD" && pox < FIELD_X-1) {
                Some(Msg::Move(code))
            } 
            else if poy == 0 || poy > FIELD_Y-1 || pox == 0 || pox > FIELD_X-1 {
                Some(Msg::Move(String::from("Edge")))
            } else {
                None
            }
        });

        let x = self.text_input.clone();
        let refocus = Callback::from(move |_event: MouseEvent| {
            let x = x.clone();
            if let Some(elem) = x.cast::<HtmlElement>() {
                elem.focus().unwrap();
            }
        });

        let reset_callback = ctx.props().reset_callback.clone();

        let columns = Children::new(
            [""].map(|ea| text(ea).to_vnode()).to_vec(),
        );

        let mut pa_iter = self.arena.clone().into_iter();

        let row_data = 0..FIELD_Y;
        let rows = row_data
            .into_iter()
            .map(|row_data| {
                tr().key(row_data.to_string())
                    .append_all({
                        let mut field: Vec<Tag> = vec![];
                        for _x in 0..FIELD_X {
                            if _x == pos_x && row_data == pos_y{
                                field.push(td().text("P").style("background-color:pink").id("current-location").node_ref(ctx.props().player_loc.clone()));
                                pa_iter.next().unwrap();
                            } else {
                                field.push(td().text(pa_iter.next().unwrap()));
                            }
                        }
                        field
                        })
            .to_vnode()})
            .collect::<Vec<_>>();

    
        let table = Table::render(columns, yew::Children::new(rows));
        
        // let pop = document().get_element_by_id("popup_container").unwrap().first_element_child().unwrap().first_element_child();
        
        let style: String = String::from("border-width: .5rem;") + "font-weight: bold;";

        html! {
            <>
            <div class="game-container">
                <div id="popup" ref={self.popup_ref.clone()}></div>
                {
                    div().node_ref(self.play_area.clone())
                    .id("playarea")
                    .class("playarea")
                    .onclick(refocus)
                    .style(style)
                    .append(table)
                }
            </div>
                <div id="game-details">
                    <p>{"Location: "}{self.x}{", "}{self.y}</p>
                    <input ref={self.text_input.clone()} 
                    id="text_input" 
                    type="text"
                    {onkeydown}
                    placeholder={"Goooooo!"} 
                    readonly=true/>
                    <div id="reset-game"><button id="reset-game" onclick={Callback::from(move |_| reset_callback.emit(()))}>{"Reset the Game"}</button></div>
                </div>
            </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        if let Some(element) = self.play_area.cast::<Element>() {

            let period = element.first_element_child().unwrap()
                            .last_element_child().unwrap()
                            .first_element_child().unwrap()
                            .first_element_child().unwrap();
            let p_box = period.get_bounding_client_rect();
            self.td_width = p_box.width() + 2.0;
            self.td_height = p_box.width() + 2.0;


            let div_rect = element.get_bounding_client_rect();
            let x1 = div_rect.width() / 2 as f64;
            let y1 = div_rect.height() / 2 as f64;

            let table_rect = element.first_element_child().unwrap().get_bounding_client_rect();
            let x2 = table_rect.width() / 2 as f64;
            let y2 = table_rect.height() / 2 as f64;

            let x_scroll: f64 = (x2-x1) as f64;
            let y_scroll: f64 = (y2-y1) as f64;
            self.lox = x_scroll as i32;
            self.loy = y_scroll as i32;

            element.set_scroll_left(self.lox);
            element.set_scroll_top(self.loy);

        }

        if let Some( elem) = self.text_input.cast::<HtmlElement>() {
            elem.focus().unwrap();
        }

    }
}


