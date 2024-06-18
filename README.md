# Wiggle

Minimal dependency manager for the Bend language. Currently supports local and
GitHub dependencies. Just add `#use {filename}` to any file that depends on
another, and it will concatenate all of your files into a single `out.bend`
with the correct ordering.

# Declaring dependencies

Wiggle uses the [EDN format](https://github.com/edn-format/edn) for configuration.
Here is an example using a local package and another from GitHub:

```clojure
{:deps
  {; local dependency (outside the current module)
   :foo {:path "../your-bend-project-here"}
   ; GitHub dependency
   :bar {:git "https://github.com/HaskellCurry/Bar"}}}
```

If you have no dependencies outside the current project, use an empty map:

```clojure
{:deps {}}
```

# Installing Wiggle

Clone from repo and install w/ cargo:

```bash
git clone https://github.com/FayCarsons/Wiggle &&
cd wiggle &&
cargo +nightly build --release &&
cargo install --path .
```

Now run `wiggle build` in the root of your project and it will output a single
`out.bend` file with all your dependencies in it.

# Plans

This is likely only going to be a toy project, I made it so people can organize
their bend projects more easily while Higher Order Co. works on an official
package manager. That said, everyone is free to use it and you're welcome
to contribute if you'd like!
