package main

import (
	"errors"
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
	HttpScheme = "http"
	WSScheme   = "ws"
)

const (
	HeartbeatInterval   = time.Second * 4
	ReconnFirstInterval = time.Second * 5
	MaxReconnInterval   = time.Hour
	WriteWait           = time.Second * 1
	PongWait            = time.Second * 6
)

type Proxier struct {
	targetHost          string //http scheme
	port                string
	checker             *NFTChecker
	hostList            []string                   //ws scheme
	wsConns             map[string]*websocket.Conn //ws scheme
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
	p := &Proxier{
		port:                port,
		checker:             checker,
		wsConns:             make(map[string]*websocket.Conn),
		heartbeatInterval:   HeartbeatInterval,
		reconnFirstInterval: ReconnFirstInterval,
		maxReconnInterval:   MaxReconnInterval,
		writeWait:           WriteWait,
		pongWait:            PongWait,
	}
	hostWsList := make([]string, len(hostList))
	for i, hostUrl := range hostList {
		hostWsList[i] = UrlToWs(hostUrl)
	}
	p.hostList = hostWsList
	p.updateTarget(hostList[0])
	return p
}

// ProxyRequestHandler handles the http request using proxy
func (s *Proxier) ProxyRequestHandler() func(http.ResponseWriter, *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		_, err := s.checker.NFTPassChecker(w, r)
		if err != nil {
			log.Info("check nft pass error", err)
			//todo: return some error code
			return
		}
		urlParse, err := url.Parse(s.targetHost)
		if err != nil {
			log.Error(err)
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
	go s.checkAlive()
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
					}
				}
			}
		}
	}
}

func (s *Proxier) tryConn(url string) {
	log.Info("try connect", url)
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
		//c.SetReadDeadline(time.Now().Add(s.pongWait * 2))
		c.SetPongHandler(func(string) error {
			c.SetReadDeadline(time.Now().Add(s.pongWait))
			return nil
		})
		go func() {
			defer func() {
				s.closeConn(url)
			}()
			for {
				_, _, er := c.ReadMessage()
				if er != nil {
					if websocket.IsUnexpectedCloseError(er, websocket.CloseGoingAway, websocket.CloseAbnormalClosure) {
					}
					log.Error(er)
					return
				}
			}
		}()
		s.wsConns[url] = c
		if s.IfTargetEmpty() {
			s.updateTarget(url)
		}
		log.Info("connected to", url)
		return
	}
}

func (s *Proxier) closeConn(url string) {
	//close conn and delete from available map
	if conn, ok := s.wsConns[url]; ok {
		conn.Close()
		delete(s.wsConns, url)
	}
	log.Warn("host closed", url)
	if UrlToWs(s.targetHost) == url {
		s.setTargetEmpty()
		for _, host := range s.hostList {
			if _, ok := s.wsConns[host]; ok {
				s.updateTarget(host)
				break
			}
		}
		if s.IfTargetEmpty() {
			log.Error("no available target host")
		}
	}
	go func() {
		s.tryConn(url)
	}()
}

func (s *Proxier) setTargetEmpty() {
	s.targetHost = ""
}

func (s *Proxier) IfTargetEmpty() bool {
	return s.targetHost == ""
}

func (s *Proxier) updateTarget(url string) {
	s.targetHost = UrlToHttp(url)
	log.Info("target host set", s.targetHost)
}
