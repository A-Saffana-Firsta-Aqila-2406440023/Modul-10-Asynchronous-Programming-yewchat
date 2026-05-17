use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="bg-gray-900 min-h-screen flex w-screen">
            <div class="container mx-auto flex flex-col justify-center items-center gap-6">
                <div class="text-center">
                    <div class="text-7xl mb-4">{"💬"}</div>
                    <h1 class="text-4xl font-bold text-white mb-2">{"Welcome to YewChat"}</h1>
                    <p class="text-gray-400 text-lg">{"Connect and chat with others in real time"}</p>
                </div>
                <form class="flex flex-col items-center gap-3 w-full max-w-sm px-4">
                    <input
                        {oninput}
                        class="w-full rounded-lg px-4 py-3 border border-gray-600 bg-gray-800 text-white placeholder-gray-400 outline-none focus:border-pink-500 transition-colors"
                        placeholder="Enter your username"
                    />
                    <Link<Route> to={Route::Chat}>
                        <button
                            {onclick}
                            disabled={username.len()<1}
                            class="w-64 px-8 py-3 rounded-lg bg-pink-500 text-white font-bold uppercase tracking-wide hover:bg-pink-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                        >
                            {"Go Chatting!"}
                        </button>
                    </Link<Route>>
                </form>
            </div>
        </div>
    }
}
