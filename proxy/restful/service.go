package restful

import (
	"github.com/ethereum/go-ethereum/cmd/utils"
	"github.com/ethereum/go-ethereum/log"
	"github.com/gorilla/mux"
	"net/http"
	cluster_client "proxy/cluster-client"
	"proxy/db"
	"proxy/snapshot"
	"time"
)

type Service struct {
	port   string
	db     *db.DBService
	client *cluster_client.ClusterClient
	chrome *snapshot.HeadlessAgent
}

func InitRestService(port string, client *cluster_client.ClusterClient, agent *snapshot.HeadlessAgent) *Service {
	return &Service{
		port:   port,
		client: client,
		chrome: agent,
	}
}

func (c *Service) Start() error {
	log.Info("start rpc port:" + c.port)
	address := "0.0.0.0:" + c.port
	r := mux.NewRouter()

	r.HandleFunc("/api/fs/profileImageUpload", func(w http.ResponseWriter, r *http.Request) {
		c.Upload(w, r)
	})

	r.HandleFunc("/api/fs/getSnapshot", func(w http.ResponseWriter, r *http.Request) {
		c.GetSnapshot(w, r)
	})

	go func() {
		err := http.ListenAndServe(address, r)
		if err != nil {
			utils.Fatalf("http listen error", err)
		}
	}()
	return nil
}

func PrintErrorStr(prefix string, detail string) string {
	if detail != "" {
		return time.Now().String() + " ERROR " + prefix + ":" + detail
	} else {
		return time.Now().String() + " ERROR " + prefix
	}
}
