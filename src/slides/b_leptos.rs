use crate::slides::{Appear, Slide};
use leptos::prelude::*;

#[component]
pub fn ViewMacro() -> impl IntoView {
    let code = r#"use crate::slides::{Appear, Slide};
use leptos::prelude::*;

#[component]
pub fn ViewMacro() -> impl IntoView {
    let code = "...";
    view! {
        <Slide title="Typecheck your HTML">
            <Appear id=1>
                <p>"Fua colega"</p>
                <code>
                    {code}
                </code>
            </Appear>
        </Slide>
    }
}"#;
    view! {
        <Slide title="Typecheck your HTML">
        <p>"Let's apply recursion on this page"</p>
        <Appear id=1>
    <div class="relative my-4 not-prose">
      <pre class="rounded-lg p-4 font-mono text-sm bg-orange-50 overflow-x-auto">
        <code class="language-rust">
        {code}
            </code>
      </pre>
      <div class="absolute top-2 right-2 bg-gray-900 px-2 py-1 rounded text-gray-200 text-sm ">
        rust
      </div>
    </div>

        </Appear>


    <div class="relative my-4 not-prose">
      <pre class="rounded-lg p-4 font-mono text-sm  overflow-x-auto">
        <code class="language-rust">
            //"use leptos::prelude::*;"
    "let a: u64 = 923;"
            </code>
      </pre>
      <div class="absolute top-2 right-2 bg-gray-900 px-2 py-1 rounded text-gray-200 text-sm ">
        rust
      </div>
    </div>
        </Slide>
    }
}
