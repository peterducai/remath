package api

import (
	"net/http"
)

func MatrixAddition(x int64, y int64) int64 {
	var z = x + y
	return z
}

func ApiMatrixAddition(w http.ResponseWriter, req *http.Request) {
	w.Header().Add("Strict-Transport-Security", "max-age=63072000; includeSubDomains")
	//w.Write([]byte(strconv.FormatInt(MatrixAddition(4323, 54325), 10)))
	w.Write([]byte(`
	<!DOCTYPE html>
<html>
<head>
<script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
<script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
</head>
<body>

<h1>Matrix Addition</h1>
<p>\[
	M=
	\begin{bmatrix}
	1 & 2 & 3 & 4 & 5 \\
	3 & 4 & 5 & 6 & 7
	\end{bmatrix}
	\]</p>

</body>
</html>
	`))
}
