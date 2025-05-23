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
      <div
        class="flex items-center justify-center w-screen h-screen"
        style="background-color: #F2B4BD; font-family: 'VT323', monospace;"
      >
        <div class="flex flex-col items-center space-y-6 bg-white bg-opacity-85 p-8 rounded-2xl shadow-xl">
          <h1
            class="text-5xl flex items-center space-x-2"
            style="font-family: 'Silkscreen', cursive; color: #333;"
          >
            <span>{"⭐️"}</span>
            <span>{"YewChat"}</span>
            <span>{"⭐️"}</span>
          </h1>
          <p class="text-lg text-gray-700 flex items-center space-x-2">
            <span>{"Connect & chat with new people around the web"}</span>
            <span>{"🌐"}</span>
          </p>

          <form class="flex items-center">
            <input
              {oninput}
              class="rounded-l-lg p-4 border-t border-b border-l text-gray-800 border-gray-200 bg-white focus:outline-none"
              placeholder="👤Username"
              style="font-family: 'VT323', monospace;"
            />
            <Link<Route> to={Route::Chat}>
              <button
                {onclick}
                disabled={username.len() < 1}
                class="flex items-center px-6 rounded-r-lg text-white font-bold p-4 uppercase border-t border-b border-r shadow-md"
                style="background-color: #F0C01D; border-color: #F0C01D; font-family: 'VT323', monospace;"
              >
                <span class="mr-2">{"📨"}</span>
                <span>{"Start Chatting"}</span>
              </button>
            </Link<Route>>
          </form>
        </div>
      </div>
    }
}