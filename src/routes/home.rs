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
        I'm a student engineer, with an interest in the fields of Computer Graphics, Low-level Development, Machine Learning, and Signal Analysis/Processing. I'm a big Rust🦀 enthusiast, also I use Arch BTW."
            br; br;
        "So welcome to my website / digital garden / virtual world! Here I'll be hosting my blog as well as any
        other silly little ideas I might wanna share with the world."
        }

        h1 { "Projects" }

        p { "These are my personal projects I'm most proud of:" }

        h2 { a href="/µwgpu"  {"µwgpu"}}
        p { "GPU microbenchmarks on any platform. It includes a library, a suite, a CLI tool for native execution, and a website for browser execution." }
        p { "Please consider submitting your execution data to support the project." }

        h2 { a href="https://github.com/zkwinkle/raytracer_ini"  {"Rust Raytracer"} }
        p { "This was my intro to computer graphics, a CPU raytracer. It was also my first semi-serious Rust project. I like showing it off because it makes pretty images." }

        h2 { a href="https://github.com/zkwinkle/Emulador-MIPS"  {"MIPS Emulator"} }
        p { "A MIPS architecture emulator written in C. It's capable of running a game I made in assembly!" }

        h1 { "Favorites" }

        p { "Here's a few of my favorites, so you can get to know me better!" }

        h3 { "Favorite videogames" }
        ul {
            li { "Hollow Knight 🪲" }
            li { "Antichamber 🧩" }
            li { "Team Fortress 2 🔥" }
            li { "Dota 2 ⚔️" }
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
