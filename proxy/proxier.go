package main

import (
	"errors"
	"fmt"
	"github.com/ethereum/go-ethereum/cmd/utils"
	"github.com/gorilla/mux"
	"github.com/gorilla/websocket"
	"net/http"
	"net/http/httputil"
	"net/url"
	"proxy/log"
	"time"
)

var (
	DialHostError = errors.New("dial host error")
)

const (
	HeartbeatInterval   = time.Second * 40
	ReconnFirstInterval = time.Second * 5
	MaxReconnInterval   = time.Hour
	WriteWait           = time.Second * 10
	PongWait            = time.Second * 60
)

type Proxier struct {
	targetHost          string
	port                string
	checker             *NFTChecker
	hostList            []string
	wsConns             map[string]*websocket.Conn
	heartbeatInterval   time.Duration
	reconnFirstInterval time.Duration
	maxReconnInterval   time.Duration
	writeWait           time.Duration
	pongWait            time.Duration
}

// NewProxy takes target host and creates a reverse proxy
func NewProxy(hostList []string, port string, checker *NFTChecker) *Proxier {
	if len(hostList) == 0 {
		log.Fatal("host url list length 0")
	}
	return &Proxier{
		targetHost:          hostList[0],
		port:                port,
		checker:             checker,
		hostList:            hostList,
		wsConns:             make(map[string]*websocket.Conn),
		heartbeatInterval:   HeartbeatInterval,
		reconnFirstInterval: ReconnFirstInterval,
		maxReconnInterval:   MaxReconnInterval,
		writeWait:           WriteWait,
		pongWait:            PongWait,
	}
}

// ProxyRequestHandler handles the http request using proxy
func (s *Proxier) ProxyRequestHandler() func(http.ResponseWriter, *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		fmt.Println("host:", r.Host)
		fmt.Println("url", r.URL.String())
		_, err := s.checker.NFTPassChecker(w, r)
		if err != nil {
			log.Info("check nft pass error", err)
			return
		}
		urlParse, err := url.Parse(s.targetHost)
		if err != nil {
			fmt.Println(err)
			return
		}
		proxy := httputil.NewSingleHostReverseProxy(urlParse)
		proxy.ServeHTTP(w, r)
	}
}

func (s *Proxier) start() {
	// handle all requests to your server using the proxy
	address := "0.0.0.0:" + s.port
	r := mux.NewRouter()
	r.PathPrefix("/").HandlerFunc(s.ProxyRequestHandler())
	go func() {
		err := http.ListenAndServe(address, r)
		if err != nil {
			utils.Fatalf("http listen error", err)
		}
	}()
}

func (s *Proxier) checkAlive() {
	go func() {
		for _, hostUrl := range s.hostList {
			s.tryConn(hostUrl)
		}
	}()

	ticker := time.NewTicker(s.heartbeatInterval)
	for {
		select {
		case <-ticker.C:
			for _, hostUrl := range s.hostList {
				if conn, ok := s.wsConns[hostUrl]; ok {
					conn.SetWriteDeadline(time.Now().Add(s.writeWait))
					if err := conn.WriteMessage(websocket.PingMessage, nil); err != nil {
						s.closeConn(hostUrl)
						s.tryConn(hostUrl)
					}
				}
			}
		}
	}
}

func (s *Proxier) tryConn(url string) {
	if _, ok := s.wsConns[url]; ok {
		return
	}
	interval := s.reconnFirstInterval
	for {
		c, _, err := websocket.DefaultDialer.Dial(url, nil)
		if err != nil {
			log.Warn(DialHostError, url)
			time.Sleep(interval)
			if interval < s.maxReconnInterval {
				interval = interval * 2
			}
			continue
		}
		go func() {
			c.SetReadDeadline(time.Now().Add(s.pongWait * 2))
			c.SetPongHandler(func(string) error {
				c.SetReadDeadline(time.Now().Add(s.pongWait))
				fmt.Println("got pong")
				return nil
			})
			_, _, er := c.ReadMessage()
			if er != nil {
				if websocket.IsUnexpectedCloseError(er, websocket.CloseGoingAway, websocket.CloseAbnormalClosure) {
					fmt.Printf("error: %v\n", er)
				}
				s.closeConn(url)
			}
		}()
		s.wsConns[url] = c
		return
	}
}

func (s *Proxier) closeConn(url string) {
	//close conn and delete from available map
	if conn, ok := s.wsConns[url]; ok {
		conn.Close()
		delete(s.wsConns, url)
	}
	if s.targetHost == url {
		for _, nextHost := range s.hostList {
			if _, ok := s.wsConns[nextHost]; ok {
				s.targetHost = nextHost
			}
		}
	}
}
