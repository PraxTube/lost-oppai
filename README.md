# Lost Oppai

## Developer Notes

### Yarn

Due to _reasons_ (look into `was_mentioned_by_*`), I have to start every dialogue with **two lines before any player option**.

Due to _reasons_ (yarn internals), I have to start every title with a non player option, this is the reason there are some weirdly placed `...` from time to time. This is most noticeable when used with the `jump` command.

### Dialogue Graph

Install `graphviz`, e.g. `sudo pacman -Syu graphviz`.

Run `cargo run -p dialogue_graph ; dot -Tsvg graphs/eleonore.dot | save o.svg -f | vieb o.svg` in nushell, translate to bash or whatever else you are using (also you probably want to replace vieb with another browser or any software that can display `.svg` files).

## Appendix

[Credits](https://github.com/PraxTube/lost-oppai/blob/master/CREDITS.md).

[License](https://github.com/PraxTube/lost-oppai/blob/master/LICENSE),
applies to everything that doesn't already have a license.
