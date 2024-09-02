use leptos::*;

#[component]
pub fn Chat() -> impl IntoView {
    view! {
        <div class="w-full h-full pb-8 flex flex-col justify-end bg-white">
            // Messages Container
            <div class="flex-grow flex flex-col justify-end gap-6 overflow-y-auto">
                // First Message
                <div class="mx-5 flex space-x-4">
                    <div class="w-12 h-12 bg-gray-300 rounded-full"></div> // Avatar
                    <div class="flex-1">
                        <div class="flex justify-between items-center">
                            <span class="font-medium text-gray-900">User</span>
                            <span class="text-sm text-gray-500">"2024-09-01 21:48"</span>
                        </div>
                        <p class="text-gray-800 mt-1">
                            "Hi! Could you please tell me which primitive I should use for rendering only on the client side? I need to render cards with text, but I also need to know the DOM width of my page, which I can only obtain from the page (it is similar to masonry js library)."
                        </p>
                    </div>
                </div>

                // Second Message
                <div class="mx-5 flex space-x-4">
                    <div class="w-12 h-12 bg-gray-300 rounded-full"></div> // Avatar
                    <div class="flex-1">
                        <div class="flex justify-between items-center">
                            <span class="font-medium text-gray-900">User</span>
                            <span class="text-sm text-gray-500">"2024-09-01 21:50"</span>
                        </div>
                        <p class="text-gray-800 mt-1">
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pa"
                        </p>
                    </div>
                </div>
            </div>

            // Input Area
            <div class="mt-5 mx-4 flex-shrink-0 flex items-center bg-white border rounded-xl">
                <input
                    type="text"
                    placeholder="Type a message..."
                    class="mx-4 flex-grow border-none focus:outline-none"
                />
                <button class="py-1 mr-3 rounded-full">
                    <img
                        src="/assets/send_icon.svg"
                        class="w-10 h-10"
                    />
                </button>
            </div>
        </div>
    }
}
