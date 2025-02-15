# Azul - Desktop GUI framework

<!-- [START badges] -->
[![Build Status Linux / macOS](https://travis-ci.org/fschutt/azul.svg?branch=master)](https://travis-ci.org/fschutt/azul)
[![Build status Windows](https://ci.appveyor.com/api/projects/status/p487hewqh6bxeucv?svg=true)](https://ci.appveyor.com/project/fschutt/azul)
[![Coverage Status](https://coveralls.io/repos/github/fschutt/azul/badge.svg?branch=master)](https://coveralls.io/github/fschutt/azul?branch=master)
[![LICENSE](https://img.shields.io/badge/license-LGPL%203.0%20+%20static%20linking-blue.svg)](LICENSE)
[![Rust Compiler Version](https://img.shields.io/badge/rustc-1.45%20stable-blue.svg)]()
<!-- [END badges] -->

> Azul is a free, functional, reactive GUI framework for Rust and C++,
built using the WebRender rendering engine and a CSS / HTML-like document
object model for rapid development of beautiful, native desktop applications

###### [Website](https://azul.rs/) | [User guide](https://azul.rs/guide) | [API documentation](https://azul.rs/api) | [Video demo](https://www.youtube.com/watch?v=kWL0ehf4wwI) | [Matrix Chat](https://matrix.to/#/#azul:matrix.org)

## About

Azul is a library for creating graphical user interfaces in Rust and C. It mixes
paradigms from functional, reactive and data-oriented programming with an API
suitable for developing cross-platform desktop applications. The two core principles
of Azul is to not render objects that aren't visible and to use composition of DOM
trees over inheritance.

Azul separates the concerns of business logic / callbacks, data model and UI
rendering / styling by not letting the UI / rendering logic have mutable access
to the application data. In Azul, rendering the view is a pure function that maps
your application data to a styled DOM. "Widgets" are just functions that render
a certain state, more complex widgets use function composition.

Since recreating DOM objects is expensive (note: "expensive" = 3 milliseconds),
Azul caches the DOM object and does NOT recreate it on every frame - only
when callbacks request to recreate it.

The application and widget data is managed using a reference-counted
boxed type (`RefAny`), which can be downcasted to a concrete type if
necessary. Widget-local data that needs to be retained between frames is
stored on the DOM nodes themselves, similar to how the HTML `dataset`
property can be used to store widget data.

## Current progress

The latest release for Windows can be found [here](https://github.com/fschutt/azul/releases/tag/1.0.0-alpha1).

Currenly I am using this framework to build cross-platform GUI applications: while the framework
isn't finished yet, it is at least possible to build native, good-looking applications
for Windows - for example, here is an Excel clone that 
[I am working on](https://gist.github.com/fschutt/8c8a139a772031bed2c4d5d62668c234):

![image](https://user-images.githubusercontent.com/12084016/126752996-1ec1f221-2b01-4f01-99c6-794640228d59.png)

Creating this demo took about two hours. As you can see, the layout system is already 
quite mature. To run this XML file, download the 
[examples.zip](https://github.com/fschutt/azul/releases/download/1.0.0-alpha1/examples.zip) 
folder and exchange the "ui.xml" file for the linked file and restart the xml.exe demo. 
The XML itself can be hot-reloaded and **later be compiled into native Rust code** - giving you both 
a fast design iteration time and native performance.

The application currently takes ~40MB to run and of course almost no CPU at all.

With the [correct CSS styles](https://gist.github.com/fschutt/9731ceec50d5fe211ab0f08082e8082f), the window is indistinguishable from a native application:

![image](https://user-images.githubusercontent.com/12084016/129535820-ca2b56a6-fdb5-4d0d-b043-a7f5394339e9.png)

![image](https://user-images.githubusercontent.com/12084016/129535780-69b9365b-ad87-439f-9d10-d416991de8fc.png)

Azul currently features:

- Text input / text entry (see "widgets" demo)
- Animations @ 60+ FPS (using [webrender](https://github.com/servo/webrender))
- CSS support [see list of supported CSS keys](https://azul.rs/guide/1.0.0-alpha1/CSSstyling)
- De- / encoding images and fonts (TTF, WOFF, OTF).
- Cross-platform text shaping (via [allsorts](https://github.com/yeslogic/allsorts)) 
- Parsing and rendering SVG (via [resvg](https://github.com/RazrFalcon/resvg))
- Rendering / embedding OpenGL content (using OpenGL textures)
- Tesselating shapes into triangles (via [lyon](https://github.com/nical/lyon))
- Asynchronously managing running background threads for File I/O
- Parsing XML (via [xmlparser](https://github.com/RazrFalcon/xmlparser))
- Stable API

Currently available widgets:

- `Button`
- `TextInput` (bug: has no cursor / text selection yet)
- `CheckBox`
- `ColorInput`
- `NumberInput`
- `ProgressBar`
- `NodeGraph`
- `Frame`
- `TabControl`

Azul also allows for easy creation of custom widgets, 
for example a [node graph widget](https://github.com/fschutt/azul/blob/master/examples/rust/nodegraph.rs)

![image](https://user-images.githubusercontent.com/12084016/128639991-e98c0b92-66df-4ad8-973b-c9d45c68d5b3.png)

All widgets are stylable via CSS. Widgets in progress:

- `ListView`
- `Spreadsheet`
- `Slider`
- `Dropdown`
- `RadioSelect`
- `RibbonBar`

Additionally, Azul features cross-platform `MsgBox` and `FileDialog` dialogs.

Caveats: 

- Currently Azul only works on Windows because of rendering problems on X11 and Cocoa
- Text shaping for non-Latin fonts as well as fallback fonts are rudimentary / non-existent
- Scrolling, especially smooth scrolling is not yet implemented entirely
- Scroll bars are not automatically inserted
- Rich Text Layout has to be calculated manually, not done automatically
- C++ API is a work in progress
- Layout engine may have bugs (but those can usually be worked around)
- Binary ABI is not entirely stable yet
- Infinite scrolling / lazy loading of DOM content is not yet supported
- Menus / context menus don't work yet (stub API)

## Installation

Due to its relatively large size (and to provide C / C++ interop),
azul is built as a dynamic library in the `azul-dll` package. You can
download pre-built binaries from [azul.rs/releases](https://azul.rs/releases).

### Using pre-built-binaries

1. Download the library from [azul.rs/releases](https://azul.rs/releases)
2. Set your linker to link against the library
    - Rust: Set `AZUL_LINK_PATH` environment variable to the path of the library
    - C / C++: Copy the `azul.h` on the release page to your project headers
        and the `azul.dll` to your IDE project.

The API for Rust, C++ and other languages is exactly the same,
since the API is auto-generated by the `build.py` script.
If you want to generate language bindings for your language,
you can generate them using the [`api.json` file](./api.json).

*To run programs on Linux, you may also need to copy the
`libazul.so` into `/usr/lib`.* Eventually this will be solved
by upstreaming the library into repositories once all major
bugs are resolved.

### Building from source (crates.io)

By default, you should be able to run

```sh
cargo install --version 1.0.0 azul-dll
```

to compile the DLL from crates.io. The library will be built
and installed in the `$AZUL_LINK_PATH` directory, which defaults to
`$CARGO_HOME_DIR/lib/azul-dll-0.1.0/target/release/`

### Building from source (git)

Building the library from source requires clang as well as
the prerequisites listed above.

```sh
git clone https://github.com/fschutt/azul
cd azul-dll
cargo build --release
```

This command should produce an `azul.dll` file in the
`/target/release` folder, in order to use this, you will
also need to set `AZUL_LINK_PATH` to `$BUILD_DIR/target/release/`.

If you are developing on the library, you may also need to
re-generate the Rust / C API, in which case you should prefer
to use the `build.py` script:

```
python3 ./build.py
```

## Example

Note: The widgets are custom to each programming language. All callbacks
have to use `extern "C"` in order to be compatible with the library.
The binary layout of all API types is described in the[`api.json` file](./api.json).

[See the /examples folder for example code in different languages](https://github.com/fschutt/azul/tree/master/examples)

![Hello World Application](https://i.imgur.com/KkqB2E5.png)

### Rust

```rust
use azul::prelude::*;
use azul_widgets::{button::Button, label::Label};

struct DataModel {
    counter: usize,
}

// Model -> View
extern "C" fn render_my_view(data: &RefAny, _: LayoutInfo) -> StyledDom {

    let mut result = StyledDom::default();

    let data = match data.downcast_ref::<DataModel>() {
        Some(s) => s,
        None => return result,
    };

    let label = Label::new(format!("{}", data.counter)).dom();
    let button = Button::with_label("Update counter")
        .onmouseup(update_counter, data.clone())
        .dom();

    result
    .append(label)
    .append(button)
}

// View updates model
extern "C" fn update_counter(data: &mut RefAny, event: CallbackInfo) -> UpdateScreen {
    let mut data = match data.downcast_mut::<DataModel>() {
        Some(s) => s,
        None => return UpdateScreen::DoNothing,
    };
    data.counter += 1;
    UpdateScreen::RegenerateDomForCurrentWindow
}

fn main() {
    let app = App::new(RefAny::new(DataModel { counter: 0 }), AppConfig::default());
    app.run(WindowCreateOptions::new(render_my_view));
}
```

### C++

```cpp
#include "azul.h"
#include "azul-widgets.h"

using namespace azul;
using namespace azul.widgets.button;
using namespace azul.widgets.label;

struct DataModel {
    counter: uint32_t
}

// Model -> View
StyledDom render_my_view(const RefAny& data, LayoutInfo info) {

    auto result = StyledDom::default();

    const DataModel* data = data.downcast_ref();
    if !(data) {
        return result;
    }

    auto label = Label::new(String::format("{counter}", &[data.counter])).dom();
    auto button = Button::with_label("Update counter")
       .onmouseup(update_counter, data.clone())
       .dom();

    result = result
        .append(label)
        .append(button);

    return result;
}

UpdateScreen update_counter(RefAny& data, CallbackInfo event) {
    DataModel data = data.downcast_mut().unwrap();
    data.counter += 1;
    return UpdateScreen::RegenerateDomForCurrentWindow;
}

int main() {
    auto app = App::new(RefAny::new(DataModel { .counter = 0 }), AppConfig::default());
    app.run(WindowCreateOptions::new(render_my_view));
}
```

### C

```c
#include "azul.h"

typedef struct {
    uint32_t counter;
} DataModel;

void DataModel_delete(DataModel* restrict A) { }
AZ_REFLECT(DataModel, DataModel_delete);

AzStyledDom render_my_view(AzRefAny* restrict data, AzLayoutInfo info) {

    AzString counter_string;

    DataModelRef d = DataModelRef_create(data);
    if (DataModel_downcastRef(data, &d)) {
        AzFmtArgVec fmt_args = AzFmtArgVec_fromConstArray({{
            .key = AzString_fromConstStr("counter"),
            .value = AzFmtValue_Uint(d.ptr->counter)
        }});
        counter_string = AzString_format(AzString_fromConstStr("{counter}"), fmt_args);
    } else {
        return AzStyledDom_empty();
    }
    DataModelRef_delete(&d);

    AzDom const html = {
        .root = AzNodeData_new(AzNodeType_Body),
        .children = AzDomVec_fromConstArray({AzDom_new(AzNodeType_Label(counter_string))}),
        .total_children = 1, // len(children)
    };
    AzCss const css = AzCss_fromString(AzString_fromConstStr("body { font-size: 50px; }"));
    return AzStyledDom_new(html, css);
}

UpdateScreen update_counter(RefAny& data, CallbackInfo event) {
    DataModelRefMut d = DataModelRefMut_create(data);
    if !(DataModel_downcastRef(data, &d)) {
        return UpdateScreen_DoNothing;
    }
    d->ptr.counter += 1;
    DataModelRefMut_delete(&d);
    return UpdateScreen_RegenerateDomForCurrentWindow;
}

int main() {
    DataModel model = { .counter = 5 };
    AzApp app = AzApp_new(DataModel_upcast(model), AzAppConfig_default());
    AzApp_run(app, AzWindowCreateOptions_new(render_my_view));
    return 0;
}
```

## Documentation

The documentation is built using the `build.py` script, which
will generate the entire `azul.rs` website in the `/target/html`
directory:

```
python3 ./build.py
```

- Class documentation is available at [azul.rs/api](https://azul.rs/api)
- Tutorials is available under [azul.rs/guide](https://azul.rs/guide).

NOTE: The class documentation can also be printed as a
PDF if you prefer that.

## License

This library is licensed under the LGPL version 3.0 with an
exception for static linking. Which means that similar to
the FLTK and wxWidgets license, you can build proprietary
applications without having to publish your source code:
you only have to publish changes made to the azul library
itself. The static linking exception allows you to statically
link Azul without having to publish your code.

Copyright 2017 - current Felix Schütt
