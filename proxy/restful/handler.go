package restful

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	cluster_client "proxy/cluster-client"
	"proxy/log"
)

type Resp struct {
	ResultCode int         `json:"resultCode"`
	ResultMsg  string      `json:"resultMsg"`
	ResultBody interface{} `json:"resultBody"`
}

func (s *Service) Upload(w http.ResponseWriter, r *http.Request) {
	rep := Resp{
		ResultCode: 500,
		ResultMsg:  "",
		ResultBody: nil,
	}

	defer func() {
		repStr, err := json.Marshal(&rep)
		if err != nil {
			log.Error(err)
			return
		}
		_, err = w.Write(repStr)
		if err != nil {
			log.Error(err)
		}
	}()

	err := r.ParseMultipartForm(10 << 20)
	if err != nil {
		log.Error(err)
		return
	}
	file, handler, err := r.FormFile("myFile")
	if err != nil {
		log.Error(err)
		return
	}
	defer file.Close()
	fmt.Printf("Uploaded File: %+v\n", handler.Filename)
	fmt.Printf("File Size: %+v\n", handler.Size)
	fmt.Printf("MIME Header: %+v\n", handler.Header)
	tempFile, err := ioutil.TempFile(cluster_client.DefaultTempFilePath, "upload-*")
	if err != nil {
		log.Error(err)
		return
	}
	defer tempFile.Close()
	fileBytes, err := ioutil.ReadAll(file)
	if err != nil {
		log.Error(err)
		return
	}

	_, err = tempFile.Write(fileBytes)
	if err != nil {
		log.Error(err)
		return
	}

	output, err := s.client.Add()
	if err != nil {
		log.Error(err)
	}

	rep.ResultMsg = "success"
	rep.ResultCode = 200
	type resp struct {
		url string
	}
	rep.ResultBody = resp{url: fmt.Sprintf("%s/ipfs/%s", s.client.Url, output.Cid.String())}
}
