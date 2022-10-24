package restful

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
	cluster_client "proxy/cluster-client"
	"proxy/log"
	"time"
)

type UrlBody struct {
	Url        string
	Html       string
	HtmlBackup string
}

type Resp struct {
	ResultCode int         `json:"resultCode"`
	ResultMsg  string      `json:"resultMsg"`
	ResultBody interface{} `json:"resultBody"`
}

//todo：check upload file attack
func (s *Service) Upload(w http.ResponseWriter, r *http.Request) {
	rep := Resp{
		ResultCode: 500,
		ResultMsg:  "",
		ResultBody: "",
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
	file, handler, err := r.FormFile("pic")
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
	defer os.Remove(tempFile.Name())
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

	output, err := s.client.Add(tempFile.Name())
	if err != nil {
		log.Error(err)
		return
	}
	time.Sleep(time.Second * 2)
	rep.ResultMsg = "success"
	rep.ResultCode = 200
	body := UrlBody{Url: fmt.Sprintf("%v/ipfs/%v", s.client.GatewayUrl, output.Cid.String())}
	bodyJson, _ := json.Marshal(&body)
	rep.ResultBody = string(bodyJson)
}

//todo：check upload file attack
func (s *Service) JsonUpload(w http.ResponseWriter, r *http.Request) {
	rep := Resp{
		ResultCode: 500,
		ResultMsg:  "",
		ResultBody: "",
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
	metadata := r.FormValue("metadata")
	if metadata == "" {
		return
	}
	tempFile, err := ioutil.TempFile(cluster_client.DefaultTempFilePath, "upload-*")
	if err != nil {
		log.Error(err)
		return
	}
	defer tempFile.Close()
	defer os.Remove(tempFile.Name())

	_, err = tempFile.Write([]byte(metadata))
	if err != nil {
		log.Error(err)
		return
	}

	output, err := s.client.Add(tempFile.Name())
	if err != nil {
		log.Error(err)
		return
	}
	time.Sleep(time.Second)
	rep.ResultMsg = "success"
	rep.ResultCode = 200
	body := UrlBody{Url: fmt.Sprintf("%v/ipfs/%v", s.client.GatewayUrl, output.Cid.String())}
	bodyJson, _ := json.Marshal(&body)
	rep.ResultBody = string(bodyJson)
}

func (s *Service) GetSnapshot(w http.ResponseWriter, r *http.Request) {
	fmt.Println(1)
	rep := Resp{
		ResultCode: 500,
		ResultMsg:  "",
		ResultBody: "",
	}
	direction := r.URL.Query().Get("sourceUrl")
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
	fmt.Println(2)
	picTempFile, err := ioutil.TempFile(cluster_client.DefaultTempFilePath, "upload-*")
	if err != nil {
		log.Error(err)
		return
	}
	defer picTempFile.Close()
	defer os.Remove(picTempFile.Name())
	//
	textTempFile, err := ioutil.TempFile(cluster_client.DefaultTempFilePath, "upload-*")
	if err != nil {
		log.Error(err)
		return
	}
	defer textTempFile.Close()
	defer os.Remove(textTempFile.Name())

	textBackup, err := ioutil.TempFile(cluster_client.DefaultTempFilePath, "upload-*")
	if err != nil {
		log.Error(err)
		return
	}
	defer textBackup.Close()
	defer os.Remove(textBackup.Name())

	fmt.Println(3)
	err = s.chrome.ShotOne(direction, picTempFile.Name(), textTempFile.Name(), textBackup.Name())
	if err != nil {
		log.Error(err)
		return
	}
	fmt.Println(4)
	picOutput, err := s.client.Add(picTempFile.Name())
	if err != nil {
		log.Error(err)
		return
	}
	textOutput, err := s.client.Add(textTempFile.Name())
	if err != nil {
		log.Error(err)
		return
	}
	textBackupOutput, err := s.client.Add(textBackup.Name())
	if err != nil {
		log.Error(err)
		return
	}
	fmt.Println(5)
	//sleep for added pic
	//time.Sleep(time.Second * 2)
	rep.ResultMsg = "success"
	rep.ResultCode = 200
	body := UrlBody{
		Url:        fmt.Sprintf("%v/ipfs/%v", s.client.GatewayUrl, picOutput.Cid.String()),
		Html:       fmt.Sprintf("%v/ipfs/%v", s.client.GatewayUrl, textOutput.Cid.String()),
		HtmlBackup: fmt.Sprintf("%v/ipfs/%v", s.client.GatewayUrl, textBackupOutput.Cid.String()),
	}
	bodyJson, _ := json.Marshal(&body)
	rep.ResultBody = string(bodyJson)
}
