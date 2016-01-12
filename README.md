# FRACTAL EXPLORER

A small project used to learn a bit of Rust.
It displays a window with a fractal, that can be explored with mouse clicks:

* Left button click: center on point and zoom in
* Right button click: center on point and zoom our
* Middle button click: center on point


Used in the Meetup group [Rust Barcelona](http://www.meetup.com/es-ES/Rust-Barcelona/).


The base code is in this repo:
https://bitbucket.org/gines_espada_almansa/fractal_explorer.git

I think we'll progress pretty fast, as all is already provided in a few snippets.

I am using Rust 1.5.0-nightly (a bit outdated) but I think it will work fine with other versions.

After we have the base code built with cargo, and our tests failing, we will add all the features orderly from these code snippets:

1.- Instructions about Mandelbrot Sets
https://bitbucket.org/snippets/gines_espada_almansa/kE7np/mandelbrot-set

2.- Mandelbrot point computed
https://bitbucket.org/snippets/gines_espada_almansa/y87Ad/mandelbrot-point

3.- Creation of a scene
https://bitbucket.org/snippets/gines_espada_almansa/AdzjR/mandelbrot-scene

4.- SDL, main function, and helpers
https://bitbucket.org/snippets/gines_espada_almansa/ayqAn/main-file-for-fractal-explorer-without

5.- Event loop and scene update
https://bitbucket.org/snippets/gines_espada_almansa/58qAG/sdl-eventpump-control