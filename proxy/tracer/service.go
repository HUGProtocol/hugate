package tracer

import (
	"errors"
	"fmt"
	"github.com/calehh/emitter"
	"os"
	"strconv"
)

type HeightManager struct {
	fileNamePrefix string
}

func NewHeightManager(fileName string, chainIdList []int64) (*HeightManager, error) {
	m := HeightManager{
		fileNamePrefix: fileName,
	}
	for _, chainId := range chainIdList {
		if _, err := os.Stat(m.fileName(chainId)); errors.Is(err, os.ErrNotExist) {
			e := m.UpdateTraceHeight(chainId, 0)
			if e != nil {
				return nil, e
			}
		}
	}

	return &m, nil
}

func (h *HeightManager) fileName(chainId int64) string {
	return h.fileNamePrefix + fmt.Sprintf("_%v.txt", chainId)
}

func (h *HeightManager) GetTraceHeight(chainId int64) (int64, error) {
	b, err := os.ReadFile(h.fileName(chainId))
	if err != nil {
		return 0, err
	}
	height, err := strconv.ParseInt(string(b), 10, 64)
	if err != nil {
		return 0, err
	}
	return height, nil
}

func (h *HeightManager) UpdateTraceHeight(chainId int64, height int64) error {
	heightStr := fmt.Sprintf("%v", height)
	return os.WriteFile(h.fileName(chainId), []byte(heightStr), 0666)
}

func NewEmitter(fileNamePrefix string, config emitter.Config, chainIdList []int64) (*emitter.EventTracer, error) {
	heightM, err := NewHeightManager(fileNamePrefix, chainIdList)
	if err != nil {
		return nil, err
	}
	return emitter.InitEventTracer(heightM, config)
}
