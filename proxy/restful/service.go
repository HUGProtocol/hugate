package restful

import (
	"github.com/ethereum/go-ethereum/cmd/utils"
	"github.com/ethereum/go-ethereum/log"
	"github.com/gorilla/mux"
	"net/http"
	cluster_client "proxy/cluster-client"
	"proxy/db"
	"time"
)

type Service struct {
	port   string
	db     *db.DBService
	client *cluster_client.ClusterClient
}

func InitRestService(port string) *Service {
	return &Service{
		port: port,
	}
}

func (c *Service) Start() error {
	log.Info("start rpc port:" + c.port)
	address := "0.0.0.0:" + c.port
	r := mux.NewRouter()

	r.HandleFunc("/profileImageUpload", func(w http.ResponseWriter, r *http.Request) {
		c.Upload(w, r)
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
