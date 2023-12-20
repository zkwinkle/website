use maud::{html, Markup};

use super::extractors::Layout;

pub async fn home(layout: Layout) -> Markup {
    layout.render( html! {
        div { div class="home-banner" {
            img src="pfp.png";
            div {
                h1 { "zkwinkle" }
                p { "Computer engineer in the making, an avid learner with too
                    many interests and not enough time."}
            }
        } }

        h1 { "Info" }
        p { "I'm Ignacio, or " em { "zkwinkle" } ", as I often go by in online spaces.
        I'm a student engineer, with an interest in the fields of Embedded (or any kind of low-level development), Graphics, Machine Learning, and Signal Analysis/Processing. I'm a big Rust🦀 enthusiast, also I use Arch BTW."
            br; br;
        "So welcome to my website / digital garden / virtual world! Here I'll be hosting my blog as well as any
        other silly little ideas I might wanna share with the world."
        }

        h1 { "Projects" }

        p { "These are my personal projects I'm most proud of:" }
        ul {
            li { a href="https://github.com/zkwinkle/website"  {"This website!"} }
            li { a href="https://github.com/zkwinkle/raytracer_ini"  {"Rust Raytracer"} }
            li { a href="https://github.com/zkwinkle/Emulador-MIPS"  {"MIPS Emulator"} }
        }

        h1 { "Favorites" }

        p { "Here's a few of my favorites, so you can get to know me better!" }

        h3 { "Favorite videogames" }
        ul {
            li { "Hollow Knight 🪲" }
            li { "Antichamber 🧩" }
            li { "Team Fortress 2 🔥" }
            li { strike{"League of Legends 🙃"} }
            li { "Zelda 🧝🏻‍♀️" }
        }

        h3 { "Favorite animes" }
        ul {
            li { "Trigun 🔫" }
            li { "Code Geass 🤖" }
            li { "Hunter x Hunter 🎣" }
            li { "Kill la Kill ✂️" }
            li { "Princess Mononoke 🐺" }
        }

    })
}
