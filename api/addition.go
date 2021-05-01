package api

import (
	"net/http"
	"strconv"

	"github.com/peterducai/remath/core"
)

func ApiAddition(w http.ResponseWriter, req *http.Request) {
	w.Header().Add("Strict-Transport-Security", "max-age=63072000; includeSubDomains")
	w.Write([]byte(strconv.FormatInt(core.Addition(4323, 54325), 10)))
}
