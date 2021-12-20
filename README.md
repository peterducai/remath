# remath

![Alt text](./static/img/remathdark.svg)

math reimagined.. CONCEPT ONLY

> buildah bud -t remath:latest .

> podman run -dt -p 7878:7878/tcp localhost/remath

```
oc new-project remath-project
oc new-app https://github.com/peterducai/remath.git
oc expose service/remath
oc status

```

see http://localhost:7878/

## Libs

Currently using:

* [Mathjax](https://github.com/mathjax/MathJax-src)
* [JSXGraph](https://github.com/jsxgraph/jsxgraph)
