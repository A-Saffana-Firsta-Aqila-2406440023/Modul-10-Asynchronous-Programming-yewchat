use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::{User, services::websocket::WebsocketService};

use crate::services::event_bus::EventBus;

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
    ToggleDarkMode,
    ToggleEmoji,
    AppendEmoji(String),
}

const EMOJIS: &[&str] = &[
    "😄" , "☺️", "😂", "🤣", "😭",
    "😆" , "🥺", "😍", "❤️", "👍",
    "🙏" , "💪", "👋", "6️⃣", "7️⃣"
];

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
    wss: WebsocketService,
    messages: Vec<MessageData>,
    _producer: Box<dyn Bridge<EventBus>>,
    username: String,
    dark_mode: bool,
    show_emoji: bool,
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

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
            username: username.to_string(),
            dark_mode: false,
            show_emoji: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    //log::debug!("got input: {:?}", input.value());
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
            Msg::ToggleDarkMode => {
                self.dark_mode = !self.dark_mode;
                true
            }
            Msg::ToggleEmoji => {
                self.show_emoji = !self.show_emoji;
                true
            }
            Msg::AppendEmoji(emoji) => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let current = input.value();
                    input.set_value(&format!("{}{}", current, emoji));
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let toggle_dark = ctx.link().callback(|_| Msg::ToggleDarkMode);
        let toggle_emoji = ctx.link().callback(|_| Msg::ToggleEmoji);

        let (bg_main, bg_sidebar, bg_user_card, border_color, bg_input, bg_msg_other, text_other, text_msg_other, bg_emoji_panel) =
            if self.dark_mode {
                ("bg-gray-900 text-white", "bg-gray-800 text-white", "bg-gray-700", "border-gray-700", "bg-gray-700 text-white placeholder-gray-400", "bg-gray-700", "text-gray-100", "text-gray-200", "bg-gray-800 border-gray-600")
            } else {
                ("bg-white text-gray-800", "bg-gray-100 text-gray-800", "bg-white", "border-gray-300", "bg-gray-100 text-gray-700", "bg-gray-100", "text-gray-800", "text-gray-600", "bg-white border-gray-200")
            };

        html! {
            <div class={format!("flex w-screen h-screen {}", bg_main)}>
                <div class={format!("flex-none w-56 h-screen {}", bg_sidebar)}>
                    <div class="text-xl p-3 font-semibold">{"Users"}</div>
                    {
                        self.users.clone().iter().map(|u| {
                            html!{
                                <div class={format!("flex m-3 {} rounded-lg p-2", bg_user_card)}>
                                    <div>
                                        <img class="w-12 h-12 rounded-full" src={u.avatar.clone()} alt="avatar"/>
                                    </div>
                                    <div class="flex-grow p-3">
                                        <div class="flex text-xs justify-between">
                                            <div>{u.name.clone()}</div>
                                        </div>
                                        <div class="text-xs text-gray-400">
                                            {"Hi there!"}
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <div class="grow h-screen flex flex-col">
                    <div class={format!("w-full h-14 border-b-2 {} flex items-center justify-between px-4", border_color)}>
                        <div class="text-xl font-semibold">{"💬 Chat!"}</div>
                        <button onclick={toggle_dark} class="px-3 py-1 rounded-full text-sm font-medium bg-pink-500 text-white hover:bg-pink-600 transition-colors">
                            { if self.dark_mode { "☀️ Light" } else { "🌙 Dark" } }
                        </button>
                    </div>
                    <div class={format!("w-full grow overflow-auto border-b-2 {}", border_color)}>
                        {
                            self.messages.iter().map(|m| {
                                let is_self = m.from == self.username;
                                let avatar = self.users.iter()
                                    .find(|u| u.name == m.from)
                                    .map(|u| u.avatar.clone())
                                    .unwrap_or_default();

                                if is_self {
                                    html!{
                                        <div class="flex flex-row-reverse items-end px-4 py-2">
                                            <img class="w-8 h-8 rounded-full ml-3 flex-shrink-0" src={avatar} alt="avatar"/>
                                            <div class="bg-pink-500 text-white p-3 rounded-tl-lg rounded-tr-lg rounded-bl-lg max-w-xs">
                                                <div class="text-xs font-semibold mb-1 opacity-80">{"You"}</div>
                                                <div class="text-sm">
                                                    if m.message.ends_with(".gif") {
                                                        <img class="mt-1 rounded" src={m.message.clone()}/>
                                                    } else {
                                                        {m.message.clone()}
                                                    }
                                                </div>
                                            </div>
                                        </div>
                                    }
                                } else {
                                    html!{
                                        <div class="flex items-end px-4 py-2">
                                            <img class="w-8 h-8 rounded-full mr-3 flex-shrink-0" src={avatar} alt="avatar"/>
                                            <div class={format!("{} p-3 rounded-tl-lg rounded-tr-lg rounded-br-lg max-w-xs", bg_msg_other)}>
                                                <div class={format!("text-xs font-semibold mb-1 {}", text_other)}>
                                                    {m.from.clone()}
                                                </div>
                                                <div class={format!("text-sm {}", text_msg_other)}>
                                                    if m.message.ends_with(".gif") {
                                                        <img class="mt-1 rounded" src={m.message.clone()}/>
                                                    } else {
                                                        {m.message.clone()}
                                                    }
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    if self.show_emoji {
                        <div class={format!("w-full p-2 border-b {} flex flex-wrap gap-1", bg_emoji_panel)}>
                            {
                                EMOJIS.iter().map(|e| {
                                    let emoji = e.to_string();
                                    let append = ctx.link().callback(move |_| Msg::AppendEmoji(emoji.clone()));
                                    html!{
                                        <button onclick={append} class="text-xl hover:bg-gray-200 hover:dark:bg-gray-600 rounded p-1 transition-colors">
                                            {e.to_string()}
                                        </button>
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    }
                    <div class="w-full h-14 flex px-3 items-center gap-2">
                        <button onclick={toggle_emoji} class="text-2xl hover:scale-110 transition-transform flex-shrink-0" title="Emoji">
                            {"😊"}
                        </button>
                        <input ref={self.chat_input.clone()} type="text" placeholder="Message" class={format!("block w-full py-2 pl-4 {} rounded-full outline-none", bg_input)} name="message" required=true />
                        <button onclick={submit} class="p-3 shadow-sm bg-blue-600 w-10 h-10 rounded-full flex justify-center items-center flex-shrink-0">
                            <svg fill="#000000" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-white">
                                <path d="M0 0h24v24H0z" fill="none"></path><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
