package main

import "net/url"

func UrlToHttp(urlStr string) string {
	u, err := url.Parse(urlStr)
	if err != nil {
		panic(err)
	}
	u.Scheme = HttpScheme
	return u.String()
}

func UrlToWs(urlStr string) string {
	u, err := url.Parse(urlStr)
	if err != nil {
		panic(err)
	}
	u.Scheme = WSScheme
	return u.String()
}
