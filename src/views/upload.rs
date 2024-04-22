use axum::response::Html;
use leptos::{
    view,
    IntoView,
    ssr::render_to_string,
};

use crate::components::meta::Metadata;

pub async fn upload() -> Html<String> {
    let html = render_to_string(|cx| view! {
        cx,
        <Metadata />
        <main class="min-h-screen bg-neutral-900 text-black flex flex-col gap-4 items-center">
            <div class="w-full p-4 shadow flex justify-center">
                <form method="post" enctype="multipart/form-data" action="/api/upload">
                    <div class="max-w-4xl w-full flex flex-col gap-4 mb-0">
                        <label class="text-xl font-medium text-white" for="input_url">
                            "Access Token:"
                        </label>
                        <input class="px-6 py-4 rounded-lg shadow"
                            type="text"
                            name="access_token"
                            size="30"
                        />

                        <label class="text-xl font-medium text-white" for="input_url">
                            "Select file:"
                        </label>
                        <input class="px-6 py-4 rounded-lg shadow text-white"
                            type="file"
                            name="file"
                            size="30"
                        />


                        <button type="submit" class="flex px-6 py-4 rounded-lg shadow bg-red-800 hover:bg-red-600 transition duration-150 text-center ease-in-out text-white font-medium">
                            "Upload"
                        </button>
                    </div>
                </form>
            </div>
        </main>
    });

    return Html(html);
}