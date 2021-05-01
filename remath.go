package main

import (
	"crypto/tls"
	"log"
	"net/http"

	"github.com/peterducai/remath/api"
)

func main() {
	mux := http.NewServeMux()
	var z = api.Addition(324, 2344)
	println(z)

	// HANDLES
	mux.HandleFunc("/addition", api.ApiAddition)
	mux.HandleFunc("/matrixaddition", api.ApiMatrixAddition)
	mux.Handle("/", http.StripPrefix("/", http.FileServer(http.Dir("./static"))))

	cfg := &tls.Config{
		MinVersion:               tls.VersionTLS12,
		CurvePreferences:         []tls.CurveID{tls.CurveP521, tls.CurveP384, tls.CurveP256},
		PreferServerCipherSuites: true,
		CipherSuites: []uint16{
			tls.TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,
			tls.TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA,
			tls.TLS_RSA_WITH_AES_256_GCM_SHA384,
			tls.TLS_RSA_WITH_AES_256_CBC_SHA,
		},
	}
	srv := &http.Server{
		Addr:         ":8000",
		Handler:      mux,
		TLSConfig:    cfg,
		TLSNextProto: make(map[string]func(*http.Server, *tls.Conn, http.Handler), 0),
	}
	log.Fatal(srv.ListenAndServeTLS("server.crt", "server.key"))
}
