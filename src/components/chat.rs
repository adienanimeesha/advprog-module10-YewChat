use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        let _ = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap());

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users = msg.data_array.unwrap_or_default();
                        self.users = users
                            .iter()
                            .map(|u| UserProfile {
                                name: u.clone(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                ),
                            })
                            .collect();
                        true
                    }
                    MsgTypes::Message => {
                        let md: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(md);
                        true
                    }
                    _ => false,
                }
            }
            Msg::SubmitMessage => {
                if let Some(input) = self.chat_input.cast::<HtmlInputElement>() {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    let _ = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap());
                    input.set_value("");
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);

        html! {
            // Root container: page bg + VT323
            <div class="flex w-screen h-screen"
                 style="background-color: #EDE1ED; font-family: 'VT323', monospace;"
            >
                // Sidebar with right border + shadow
                <div class="flex-none w-56 h-full overflow-auto border-r border-gray-300 shadow-sm"
                     style="background-color: #F9D0CE;"
                >
                    <div class="text-xl p-3">{"Users"}</div>
                    {
                        self.users.iter().map(|u| {
                            html! {
                                <div class="flex items-center m-3 p-2 rounded-lg shadow-inner"
                                     style="background-color: #FBE8EB;"
                                >
                                    <img
                                        class="w-12 h-12 rounded-full mr-3"
                                        src={u.avatar.clone()}
                                        alt="avatar"
                                    />
                                    <div>
                                        <div class="text-sm">{ &u.name }</div>
                                        <div class="text-xs text-gray-500">{"Hi there!"}</div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>

                // Main chat area
                <div class="grow flex flex-col">
                    <div class="w-full h-14 flex items-center px-6 border-b border-gray-300 shadow-sm"
                         style="background-color: #FEF8E7;"
                    >
                        <span style="font-family: 'VT323', monospace; font-size:1.5rem;">
                            {"💬 Welcome to "}
                        </span>
                        <span style="font-family: 'Silkscreen', cursive; font-size:1.75rem;">
                            {"₊ ⊹YewChatˎˊ˗!"}
                        </span>
                    </div>

                    // Message list
                    <div class="grow overflow-auto p-4 space-y-4">
                        {
                            self.messages.iter().map(|m| {
                                let user = self.users.iter().find(|u| u.name == m.from).unwrap();
                                html! {
                                    <div class="flex items-start max-w-3xl rounded-xl p-3 shadow-inner"
                                         style="background-color: #F9F2F7;"
                                    >
                                        <img
                                            class="w-8 h-8 rounded-full mr-3"
                                            src={user.avatar.clone()}
                                            alt="avatar"
                                        />
                                        <div>
                                            <div class="text-sm font-semibold">{ &m.from }</div>
                                            <div class="text-xs text-gray-600 mt-1">
                                                if m.message.ends_with(".gif") {
                                                    <img class="mt-2 rounded" src={m.message.clone()}/>
                                                } else {
                                                    { &m.message }
                                                }
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>

                    <div class="w-full h-16 flex items-center px-6 border-t border-gray-300 shadow-sm"
                         style="background-color: #FEF8E7;"
                    >
                        <input
                            ref={self.chat_input.clone()}
                            type="text"
                            placeholder="Message"
                            class="flex-grow py-2 px-4 rounded-full mr-4 focus:outline-none"
                            style="background-color: #FFFFFF; font-family: 'VT323', monospace;"
                        />
                        <button
                            onclick={submit}
                            class="w-12 h-12 rounded-full flex items-center justify-center shadow-lg"
                            style="background-color: #F0C01D;"
                        >
                            <span style="font-size:1.25rem;">{"📨"}</span>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}