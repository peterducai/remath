# remath

![Alt text](./static/img/remathdark.svg)

math reimagined.. CONCEPT ONLY

# build

> cargo build

# build container

> buildah bud -t remath:latest .

> podman run -dt -p 7878:7878/tcp localhost/remath

see http://localhost:7878/

# run in Openshift

You can download [CRC](https://console.redhat.com/openshift/create/local) to run local OCP 4.

```
oc new-project remath-project
oc new-app https://github.com/peterducai/remath.git
oc expose service/remath
oc status  (until build is finished)
oc get route
```



## Libs

Currently using:

* [Mathjax](https://github.com/mathjax/MathJax-src)
* [JSXGraph](https://github.com/jsxgraph/jsxgraph)
