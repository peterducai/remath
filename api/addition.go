package api

import (
	"net/http"
	"strconv"
)

func Addition(x int64, y int64) int64 {
	var z = x + y
	return z
}

func ApiAddition(w http.ResponseWriter, req *http.Request) {
	w.Header().Add("Strict-Transport-Security", "max-age=63072000; includeSubDomains")
	w.Write([]byte(strconv.FormatInt(Addition(4323, 54325), 10)))
}
