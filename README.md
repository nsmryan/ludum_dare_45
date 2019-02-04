# Writing a Rust Roguelike for Desktop and the Web

This is a little proof of concept for a roguelike written in pure Rust
which can be easily compiled for Windows, macOS Linux and the web
browser (via WebAssembly).

You can clone the repo and run it or follow the guide below which will
build it from scratch.


## Motivation

Rust is a pretty good language for writing roguelikes: it is fast (on
par with C or C++), modern (modules, strings, closures, algebraic data
types) and it can build standalone executables for all the major
platforms as well as the web!

The latter makes is particularly nice for game jams -- you don't have
to worry about software packaging and distribution. Just give people
the link and they can play your game.

We're going to build a bare-bones skeleton for a traditional ASCII
roguelike that you can take and turn into a real game. The same
codebase will work for all the major desktop platforms as well as the
web. And it will support multiple fonts and text sizes so you can have
your square maps and readable text *at the same time*!

It will look like this:

![./screenshot.png]


## Setup

### Rust

You will need the 2018 edition of Rust to get started. Get it from the
Rust website:

https://www.rust-lang.org/learn/get-started

> If you want to build the Web version, you'll want to install
> `rustup`. If you only care about desktop, feel free to get the
> standalone version via your package manager or installer.


### WebAssembly

The WebAssembly compilation target is not shipped by default, but you
can add it like so:

    $ rustup target add wasm32-unknown-unknown

> You don't need this if you only care about the desktop.

### cargo-web

[cargo-web](https://github.com/koute/cargo-web) is used to create
everything you need to ship your game on the web. Install it like so:

    $ cargo install cargo-web

> You don't need this if you only care about the desktop.


### Repository

Create a new project like so:

    $ cargo new --vcs git quicksilver-roguelike

> This will initialise a new git repository as well! Remove `--vcs
> git` if you don't want that.

If you've got everything set up properly, you should be able to run
the default program:

    $ cd quicksilver-roguelike
    $ cargo run --release

It will print `Hello, world!`.


## Quicksilver

Rust has several gamedev engines and frameworks. We are going to
use [Quicksilver](https://www.ryanisaacg.com/quicksilver/) because it
lets you target both the desktop and web really easily!

Open your `Cargo.toml` in the root of your repository and add this
below `[dependencies]`:

```toml
# More features: "collisions", "complex_shapes", "immi_ui", "sounds", gamepads
quicksilver = { version = "0.3.6", default-features = false, features = ["fonts", "saving"]}
```

We're disabling most of the features. You don't have to do this, but
some of these require libraries you might not have installed. The
above should compile pretty much anywhere.

You can always add the sound or gamepad support later if you need it.

Run the program again to build the quicksilver dependency:

    $ cargo run --release

This might take a couple of minutes and then print out the same
text as before.

> We'll be always building the optimised version in this guide. You
> can drop the `--release` flag, but you're not allowed to make any
> speed comparisons. Rust's debug builds are slower than you think.
> They're slower than unoptimised C++. They're slower than Ruby.

## Hello, Web!

First things we'll do is create a window and print some text on it.
We'll do all our coding in the `src/main.rs` file in your repository.

### Empty Window

A Quicksilver app is all encapsulated in an item that implements the
`quicksilver::lifecycle::State` trait:

```rust
struct Game;

impl State for Game {
    /// Load the assets and initialise the game
    fn new() -> Result<Self> {
        Ok(Self)
    }

    /// Process keyboard and mouse, update the game state
    fn update(&mut self, window: &mut Window) -> Result<()> {
        Ok(())
    }

    /// Draw stuff on the screen
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        Ok(())
    }
}
```

We will add our actual game code in the `new`, `update` and `draw`
methods later.

To run the game, replace the `main` function with:

```rust
fn main() {
    let settings = Settings {
        ..Default::default()
    };
    run::<Game>("Quicksilver Roguelike", Vector::new(800, 600), settings);
}
```

The `Settings` struct lets you control various engine settings that
we'll get to later. The `800` and `600` numbers represent the
**logical** size of your window. Depending on your DPI, it might be
bigger than that.

And finally, we need to bring all the items we use into scope. Put
this on top of your file:

```rust
use quicksilver::{
    geom::Vector,
    lifecycle::{run, Settings, State, Window},
    Result,
};
```

Running the game now should produce an empty window filled with black:

    $ cargo run --release

![Empty Window](screenshots/01_empty_window.png)

### Assets

You may have noticed the following message when running the code:

```
Warning: no asset directory found. Please place all your assets inside
a directory called 'static' so they can be loaded
Execution continuing, but any asset-not-found errors are likely due to
the lack of a 'static' directory.
```

Quicksilver expects all the game assets to be in the `static`
directory under the project's root. We don't have one, hence the
warning.

Let's create it:

    $ mkdir static

The warning should disappear.

What goes into assets? Sounds, images, models, maps and anything else
your game will need to load. Including fonts.

Let's download [mononoki](https://madmalik.github.io/mononoki/), a
beautiful little monospace font. You can get it from:

https://madmalik.github.io/mononoki/

It is an [open source](https://github.com/madmalik/mononoki) font [distributed under the Open Font License 1.1](https://github.com/madmalik/mononoki/blob/master/LICENSE).

Get the `mononoki-Regular.ttf` file and copy it into our new `static`
directory.


### Loading the font

All initial asset loading should happen in the `new` function. We will
load the font file, use it to render text into an image and store it
in our `Game` struct:

```rust
fn new() -> Result<Self> {
    let font_mononoki = "mononoki-Regular.ttf";

    let title = Asset::new(Font::load(font_mononoki).and_then(|font| {
        font.render("Quicksilver Roguelike", &FontStyle::new(72.0, Color::BLACK))
    }));

    let mononoki_font_info = Asset::new(Font::load(font_mononoki).and_then(|font| {
        font.render(
            "Mononoki font by Matthias Tellen, terms: SIL Open Font License 1.1",
            &FontStyle::new(20.0, Color::BLACK),
        )
    }));

    Ok(Self)
}
```

> Thanks, Matthias!

As you can see, we're rendering two bits of text: a heading 72 points
big and a little licensing information about our font.

`Font::load` returns a `Future`. It exits immediately and load the
actual file in the background. Calling `and_then` will let us
manipulate the value (font) when it's ready.

Yep, Quicksilver does dynamic asset loading for us!

`font.render` takes a text and a style (font size & colour) and
creates an image we can draw on the screen.

We could just always call `font.render` in the `draw` function and
output the image directly, but font processing is slower than
just drawing an image. It has to rasterise the glyphs, handle kerning,
etc. So we only do that work once and `draw` a static image later.

We'll store both images in the `Game` struct:

```rust
struct Game {
    title: Asset<Image>,
    mononoki_font_info: Asset<Image>,
}
```

And return them from `new`:

```rust
Ok(Self {
    title,
    mononoki_font_info,
})
```

We well need to add some more imports to compile:

```rust
use quicksilver::{
    geom::Vector,
    graphics::{Color, Font, FontStyle, Image},
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};
```

(we've added the `graphics` section, `lifecycle::Asset` and `Future`).


### Drawing text

The text is black, so we need to change the window's background to
make it visible. Let's do that and draw the text:

```rust
fn update(&mut self, window: &mut Window) -> Result<()> {
    window.clear(Color::WHITE)?;

    self.title.execute(|image| {
        window.draw(
            &image
                .area()
                .with_center((window.screen_size().x as i32 / 2, 40)),
            Img(&image),
        );
        Ok(())
    })?;

    self.mononoki_font_info.execute(|image| {
        window.draw(
            &image
                .area()
                .translate((2, window.screen_size().y as i32 - 60)),
            Img(&image),
        );
        Ok(())
    })?;

    Ok(())
}
```

`window.clear(color)` is quite straightforward, but what's the deal
with this `execute` closure business?

We're not storing the images directly -- we're storing them as
Futures. That is, values that might not actually exist yet (because
the font was not loaded yet).

To get to an asset, we need to call execute and pass in a closure that
operates on that asset (an `Image` in our case). If it's loaded, the
closure will be called, if not, nothing will happen (but the program
will keep going).

Inside the closure we call `window.draw` which takes two parameters: a
`Rectangle` to with the position and size and the thing that we
actually want to draw.

That can be either an image (`Background::Img`), colour fill
(`Background::Col`) or a combination of the two
(`Background::Blended`).

We're drawing images so we use `Img(&image)`.

`Image::area()` will give you the position and size of your image.
That would draw both on the top-left corner however (`x: 0, y: 0`).

So we use `.with_center` to draw the title centered near the top of
the screen and `.translate` to draw the message at the bottom.

Let's add the missing imports and see the result:

```rust
use quicksilver::{
    geom::{Shape, Vector},
    graphics::{Background::Img, Color, Font, FontStyle, Image},
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};
```

We've added the `geom::Shape` trait for the `with_center` and
`translate` methods as well as `graphics::Background::Img`.

Let's run it:

    $ cargo run --release

![First text](screenshots/02_first_text.png)

*ugh*

Depending on your system's DPI settings, the text may look okay or
weirdly pixelated like above.

### Font rendering artefacts

If you see the artefacts (bear in mind that even if you don't your
users might), they're caused by a combination two things:

1. Window scaling due to DPI
2. Quicksilver's default image scaling strategy (pixelate)

You can read more about DPI here:

https://docs.rs/glutin/0.19.0/glutin/dpi/index.html

If your system is configured for a DPI that's say `1.3`, the window
size (with all its contents) will be scaled up to it. This is a very
important accessibility feature and not handling it properly can make
your game too small for people with bad eyesight or a "Retina
display".

The problem here isn't the DPI itself, but how the image gets
stretched.

By default, Quicksilver uses the Pixelate scale strategy which tries
to preserve the individual pixels. This looks great at 2X, 3X etc.
scales, but not so much at a 1.3X. Especially for text rendering.

We can switch to the `Blur` strategy instead. In `main`:

```rust
let settings = Settings {
    scale: quicksilver::graphics::ImageScaleStrategy::Blur,
    ..Default::default()
};
```

Surprise surprise, it looks blurry:

![Blurry text](screenshots/03_blurry_text.png)

But it is still readable.

If you want to have a full control over your the window and text size,
add this line at the beginning of your `main` function:

```rust
std::env::set_var("WINIT_HIDPI_FACTOR", "1.0");
```

That will force the DPI to be 1.0. Games are more sensitive to scaling
than other application due to their pixel-based visual nature, so this
can be ok. However you ought to provide a way of scaling the UI from
within your game in that case! Ideally, defaulting to the system's DPI value.

![Crisp text](screenshots/04_crisp_text.png)

> We will still keep the `Blur` scaling strategy. Quicksilver's
> coordinates are floating numbers and things like `with_center` can
> easily result in non-integer coordinates. Again, these tend to look
> better with Blur.

## Generating the game map

Okay, let's build the game map and add some entities to it.

The map will be a `Vec<Tile>`:

```rust
#[derive(Clone, Debug, PartialEq)]
struct Tile {
    pos: Vector,
    glyph: char,
    color: Color,
}
```

The `pos` is a `quicksilver::geom::Vector` -- no need to invent our
own.

> You might still want to do it, e.g. to prevent mixing window and map
> coordinates at compile time.

A proper roguelike would use a procedural / random generation to build
the map. We'll just create an empty rectangle with `#` as its edges:

```rust
fn generate_map(size: Vector) -> Vec<Tile> {
    let width = size.x as usize;
    let height = size.y as usize;
    let mut map = Vec::with_capacity(width * height);
    for x in 0..width {
        for y in 0..height {
            let mut tile = Tile {
                pos: Vector::new(x as f32, y as f32),
                glyph: '.',
                color: Color::BLACK,
            };

            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                tile.glyph = '#';
            };
            map.push(tile);
        }
    }
    map
}
```

And well add some entities as `Vec<Entity>`:

```rust
#[derive(Clone, Debug, PartialEq)]
struct Entity {
    pos: Vector,
    glyph: char,
    color: Color,
    hp: i32,
    max_hp: i32,
}

fn generate_entities() -> Vec<Entity> {
    vec![
        Entity {
            pos: Vector::new(9, 6),
            glyph: 'g',
            color: Color::RED,
            hp: 1,
            max_hp: 1,
        },
        Entity {
            pos: Vector::new(2, 4),
            glyph: 'g',
            color: Color::RED,
            hp: 1,
            max_hp: 1,
        },
        Entity {
            pos: Vector::new(7, 5),
            glyph: '%',
            color: Color::PURPLE,
            hp: 0,
            max_hp: 0,
        },
        Entity {
            pos: Vector::new(4, 8),
            glyph: '%',
            color: Color::PURPLE,
            hp: 0,
            max_hp: 0,
        },
    ]
}
```

Now let's hook them up to our `Game` struct:

```rust
struct Game {
    title: Asset<Image>,
    mononoki_font_info: Asset<Image>,
    map_size: Vector,
    map: Vec<Tile>,
    entities: Vec<Entity>,
}
```

We're adding the map size as well -- that will come in handy later.

Next, call both functions in `Game::new`:

```rust
let map_size = Vector::new(20, 15);
let map = generate_map(map_size);
let mut entities = generate_entities();
```

And make sure we actually return the new fields:

```rust
Ok(Self {
    title,
    mononoki_font_info,
    map_size,
    map,
    entities,
})
```

We need to add the player, too (represented, as always, by the `@`
symbol)!

Having all the entities (monsters, items, NPCs, player, etc.) in one
place (the `entities` Vec) is quite useful, but the player entity is
always a little special. We need to read it to show its health bar,
update it's position on key presses, etc.

So let's also save the player's index so we can look them up any time
we want.

Put this in `Game::new` right after the `generate_entities()` call:

```rust
let player_id = entities.len();
entities.push(Entity {
    pos: Vector::new(5, 3),
    glyph: '@',
    color: Color::BLUE,
    hp: 3,
    max_hp: 5,
});
```

The player will have a blue colour and they will *not* be fully healed
(so we can see write a nice two-colour health bar later).

Add `player_id: usize` to our `Game` definition:

```rust
struct Game {
    title: Asset<Image>,
    mononoki_font_info: Asset<Image>,
    map_size: Vector,
    map: Vec<Tile>,
    entities: Vec<Entity>,
    player_id: usize,
}
```

And return it at the end of `new`.

## Building the tilemap

How do we draw the individual characters? We could call `font.render`
in our `draw` function, but that would be really slow.

What we'll do instead is to build a texture atlas -- an image
containing all our glyphs (the player, monsters, wall) and then render
parts of it on the screen.

Again, rendering an image is much faster than drawing a character from
a font file.

First, we need to create all the characters we're going to render. Put
this in `Game::new` after our entity code:

```rust
let game_glyphs = "#@g.%";
```

These are the characters we're going to use. A bigger game will have
more of these and you may want to generate them in your code instead
of hardcoding like here.

Since Quicksilver is able to build an `Image` from a string slice (we
did that already for our game title), we can just reuse that.

You can then call the `subimage` method, pass it a `Rectangle` with
the position and size you want and it will give you a new `Image`
back.

> This will *not* clone the entire image. It uses a reference-counted
> pointer back to the source, so the operation is quick and doesn't
> take up a lot of memory.

We could either call `subimage` directly in our `draw` function, or we
could generate a sub-image once for each glyph and then just reference
those when drawing.

We're going to do the latter and we'll use a `HashMap` (gasp!) to get
from a `char` to the corresponding `Image`.

> There's a ton of other ways to do this. For example: create an image
> of all ASCII characters and then have a `Vec<Image>` for each
> subimage. Each image's index would be its ASCII value. This would
> probably be faster, but it would waste a little more memory and
> you'll need to check that your `char` (a 32-bit Unicode value) can
> be converted to the right range. Also, what if you want to add some
> good-looking Chinese characters? You should measure and make your
> own trade-offs.

Okay! Let's add our tile size. The font is twice as tall as it is
wide, so 24x12 pixels should do nicely:

```rust
let tile_size_px = Vector::new(12, 24);
```

> If you don't use different types for different units, it's a good
> idea to at least put them in the name so you don't mix them up by
> accident.

And build the tilemap:

```rust
let tilemap = Asset::new(Font::load(font_mononoki).and_then(move |text| {
    let tiles = text
        .render(game_glyphs, &FontStyle::new(tile_size_px.y, Color::WHITE))
        .expect("Could not render the font tilemap.");
    let mut tilemap = HashMap::new();
    for (index, glyph) in tilemap_source.chars().enumerate() {
        let pos = (index as i32 * tile_size_px.x as i32, 0);
        let tile = tiles.subimage(Rectangle::new(pos, tile_size_px));
        tilemap.insert(glyph, tile);
    }
    Ok(tilemap)
}));
```

The beginning is the same as our other font-rendering: we load the
font and build the image.

The rest creates a new `HashMap` and then creates a new sub-image for
every glyph.

> This relies on the fact that every glyph has the same width. In
> other words, it only works for monospace fonts such as Mononoki.
> If you want to use a proportional font (say Helvetica), you will
> need to build the font-map yourself. You can use the `rusttype`
> library to do it.

Add the `tilemap` and `tile_size_px` to `Game`:

```rust
struct Game {
    title: Asset<Image>,
    mononoki_font_info: Asset<Image>,
    map_size: Vector,
    map: Vec<Tile>,
    entities: Vec<Entity>,
    player_id: usize,
    tilemap: tilemap: Asset<HashMap<char, Image>>,
    tile_size_px: Vector,
}
```

and return them from `new`.

We need to add `quicksilver::geom::Rectangle` and
`std::collections::HashMap` to our imports:

```rust
use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{Background::Img, Color, Font, FontStyle, Image},
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};

use std::collections::HashMap;
```


## Drawing the map

We've got the map and the tiles, let's put them to use!

Drawing the map is easy: we calculate the position of each tile, grab
the corresponding image and draw it on the window. Since `tilemap` is
an Asset / Future, this must happen inside an `execute` block:

```rust
fn draw(&mut self, window: &mut Window) -> Result<()> {
    let tile_size_px = self.tile_size_px;

    let (tilemap, map) = (&mut self.tilemap, &self.map);
    tilemap.execute(|tilemap| {
        for tile in map.iter() {
            if let Some(image) = tilemap.get(&tile.glyph) {
                let pos_px = tile.pos.times(tile_size_px);
                window.draw(
                    &Rectangle::new(pos_px, image.area().size()),
                    Blended(&image, tile.color),
                );
            }
        }
        Ok(())
    })?;

    Ok(())
}
```

If we called `self.tilema.excute` directly, it would mutably borrow
the entire `Game` struct and we wouldn't be able to access `self.map`
or `self.tile_size_px`. So we do a partial borrow and call `execute`
on that.

> Try removing the `let` lines and use `self.map` etc. in the `draw`
> function. See what happens!

The `Vector::times` method multiplies the corresponding Vector
elements. So `v1.times(v2)` is the same as: `Vector::new(v1.x * v2.x,
v1.y * v2.y)`. This gets us from the tile position (from `0` to `20`)
to the pixel position on the screen (from `0` to `240`).

> There are a few different ways to multiply vectors in Maths, so
> they're all available as separate methods instead of the asterisk
> operator.

And finally, the `Blended` background option allows us to apply a
colour to whatever's on the picture. Since our glyphs are white, this
turns them into whatever colour we set.

We need to add it to our `use` statement:

```rust
quicksilver::graphics::Background::Blended
```

And that should do it:

![Tilemap in the top-left corner](05_corner_tilemap.png)

Looking good, but the map is in the top-left corner, obscured by the
title text! Let's fix that.

We'd like to move the whole map further down and to the right. That
means shifting each tile that we draw. Let's say 50 pixels to the
right and 150 down.

```rust
let offset_px = Vector::new(50, 120);
```

And then in `window.draw` we'll add `offset_px` to `pos_px ` in the
`Rectangle::new` call:

```rust
window.draw(
    &Rectangle::new(offset_px + pos_px, image.area().size()),
    Blended(&image, tile.color),
);
```

![Offset map](06_offset_map.png)

Better.


## Adding square font

This starts looking closer to a real roguelike, but *can* improve upon
it. Why are the tiles not square? Personal preference aside (whatever
floats your boat), in our case it's just an artefact of the font we're
using.

We've picked Mononoki, because we* like it! It's not the perfect font
for text legibility (proportional-width would work better), but it's
good enough and it feels right to use a monospace font in a roguelike.

> *we == I, Tomas Sedovic. I like Mononoki. It's awesome.

But it's not a square font.

If we were writing a terminal game or using a library that emulates
one (such as libtcod), that would be that. Everything would have to be
the same font and you'd have to choose between a square font (good for
the map, bad for text) or a non-square one (good for text, bad for the
map).

Neither is a great option, but all old-school roguelikes were that
way.

We *can* do (arguably) better, however!

Let's just pick a second font with square proportions and use that for
the map (and keep doing text with Mononoki).

For a font with square proportions, you clearly can't do better than
Square:

http://strlen.com/square/?s[]=font

It's licensed under CC BY 3.0. Download it and put `square.ttf` in the
`static` folder.

> You can also just keep using a non-square font and simply center
> each glyph into a square tile. I did that in my first game. It's
> fine.

We'll need to tweak a few things in `Game::new`. We'll add the new font
file name and then replace `font_mononoki` in the `tilemap`'s
`Font::load` with `font_square`:

```rust
let font_square = "square.ttf";
let game_glyphs = "#@g.%";
let tile_size_px = Vector::new(12, 24);
let tilemap = Asset::new(Font::load(font_mononoki).and_then(move |text| {
    let tiles = text
        .render(game_glyphs, &FontStyle::new(tile_size_px.y, Color::WHITE))
        .expect("Could not render the font tilemap.");
    let mut tilemap = HashMap::new();
    for (index, glyph) in game_glyphs.chars().enumerate() {
        let pos = (index as i32 * tile_size_px.x as i32, 0);
        let tile = tiles.subimage(Rectangle::new(pos, tile_size_px));
        tilemap.insert(glyph, tile);
    }
    Ok(tilemap)
}));
```

You might wonder whether we should also update `tile_size_px`. We
should! Look what happens if we don't:

![Half square](screenshots/07_half_square.png)

> Glitches like these are one of gamedev's underrated pleasures.

Interesting, but not *quite* what we want. Make the tile size a proper
square:

```rust
let tile_size_px = Vector::new(24, 24);
```

![Square map](screenshots/08_square_map.png)

Take that, 1950s terminals!

> Z3, one of the first computers with a terminal had *1408 bits* of
> data memory. Our tilemap image *alone* has 9216 **bytes**.


## Square credit

Since we've added another font, let's show our appreciation to its
author too!

In `Game::new`:

```rust
let square_font_info = Asset::new(Font::load(font_mononoki).and_then(move |font| {
    font.render(
        "Square font by Wouter Van Oortmerssen, terms: CC BY 3.0",
        &text_style,
    )
}));
```

Add it to the `Game` struct:

```
square_font_info: Asset<Image>,
```

And then in `Game::draw`:

```rust
self.square_font_info.execute(|image| {
    window.draw(
        &image
            .area()
            .translate((2, window.screen_size().y as i32 - 30)),
        Img(&image),
    );
    Ok(())
})?;
```

![Square font credits](10_square_font_credits.png)

Thanks a bunch, Wouter!


## Adding entities

Now that our map looks the way we want it, let's add the entities. It
looks pretty much identical to how we draw the map:

```rust
let (tilemap, entities) = (&mut self.tilemap, &self.entities);
tilemap.execute(|tilemap| {
    for entity in entities.iter() {
        if let Some(image) = tilemap.get(&entity.glyph) {
            let pos_px = offset_px + entity.pos.times(tile_size_px);
            window.draw(
                &Rectangle::new(pos_px, image.area().size()),
                Blended(&image, entity.color),
            );
        }
    }
    Ok(())
})?;
```

![Entities!](screenshots/09_entities.png)

We can see the player (`@`) a couple of (definitely friendly) goblins
(`g`) and some purple food (`%`). Time to party!

You might also notice that the dots representing empty space are still
visible. The images are just drawn on top of one another so if they
don't cover something perfectly, it will shine through.

> Spoiler alert: we will not fix that here. It's your first homework!


## Health bar

One final piece of distinguished visual art: our protagonist's health bar!

We're going to get the player's entity, set the full bar's width at a
hundred pixels and calculate how much of it should we show based on
the player's hit points:

```rust
let player = &self.entities[self.player_id];
let full_health_width_px = 100.0;
let current_health_width_px =
    (player.hp as f32 / player.max_hp as f32) * full_health_width_px;
```

Next, let's calculate its position. Let's put it to the right side of
the map. That means getting the map's size in pixels plus the offset:

```rust
let map_size_px = self.map_size.times(tile_size_px);
let health_bar_pos_px = offset_px + Vector::new(map_size_px.x, 0.0);
```

And finally draw it. First we draw the full width in a somewhat
transparent colour and then the current value in full red:


```rust
// Full health
window.draw(
    &Rectangle::new(health_bar_pos_px, (full_health_width_px, tile_size_px.y)),
    Col(Color::RED.with_alpha(0.5)),
);

// Current health
window.draw(
    &Rectangle::new(health_bar_pos_px, (current_health_width_px, tile_size_px.y)),
    Col(Color::RED),
);
```

And we need to `use` `quicksilver::graphics::Background::Col`. That's
the final `Background` value -- representing the whole area filled
with the given colour.

![Health bar](screenshots/11_health_bar.png)


## Move the player around

Games have to be interactive. Let's move our player if any of the
arrow keys are pressed:

```rust
/// Process keyboard and mouse, update the game state
fn update(&mut self, window: &mut Window) -> Result<()> {
    use quicksilver::input::ButtonState::*;

    let player = &mut self.entities[self.player_id];
    if window.keyboard()[Key::Left] == Pressed {
        player.pos.x -= 1.0;
    }
    if window.keyboard()[Key::Right] == Pressed {
        player.pos.x += 1.0;
    }
    if window.keyboard()[Key::Up] == Pressed {
        player.pos.y -= 1.0;
    }
    if window.keyboard()[Key::Down] == Pressed {
        player.pos.y += 1.0;
    }
    Ok(())
}
```

Straightforward stuff.

Finally, you can quit the game from your program by calling
`window.close()`:

```
if window.keyboard()[Key::Escape].is_down() {
    window.close();
}
```

> Please make sure you don't ship your game with this left in! Someone
> will press Escape unintentionally and lose their progress (or at
> least be annoyed they have to start it again if it was saved). That
> someone will be me. Please add a confirmation step.

We'll need to add `quicksilver::input::Key` to our `use` declarations.


As you can see, the player can walk through everything. The goblins,
food, even the walls! This is fine if you're making a roguelike where
you're a ghost, but probably not in most other circumstances.

> We're not going to fix that here either! Your second homework.


## Web Version

One last thing.

Running the game with `cargo run` builds the desktop version.

Run `cargo web start --auto-reload` and go to

http://localhost:8000

![WebAssembly](screenshots/12_web_assembly.png)

It works! And it looks just like the desktop version. No changes
necessary.

If you run this:

```
$ cargo web deploy
```

Everything will be added to your `target/deploy` directory. Upload it
on the web and give people the link!


## Make your own game

This is where *we* end, but *you* are just beginning!

We've built something that has a lovely square map, readable text,
filled rectangles and runs on Windows, macOS, Linux and the *freaking
web*!

But it's not a real roguelike yet. In addition to the homeworks, it's
mising some of these things:

* procedural map generation
* collision handling
* monsters
* AI
* combat
* items

Plus whatever unique twists you want to do! Go make games!


## TODO:

* bring the changes here back to the quicksilver-roguelike repo
* deploy the web version to github pages
* go through all this & edit it
* publish it on my blog