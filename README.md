
# Wiggle

Minimal dependency manager for the Bend language. Currently supports local and
GitHub dependencies. Just add `#use {filename}` to any file that depends on
another, and it will concatenate all of your files into a single `out.bend`
with the correct ordering.

# Declaring dependencies

Wiggle uses the [edn format](https://github.com/edn-format/edn) for configuration.
Here is an example using a local package and another from GitHub:

```clojure
{:deps {
  :foo {:path "path/to/local/project"}
  :bar {:git "https://github.com/HaskellCurry/Bar"}
}}
```

# Plans

This is likely only going to be a toy project, I made it so people can organize
their bend projects more easily while Higher Order Co. works on an official
package manager. That said, everyone is free to use it and you're welcome
to contribute if you'd like!
