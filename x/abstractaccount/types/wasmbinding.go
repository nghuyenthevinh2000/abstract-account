package types

import (
	"github.com/CosmWasm/wasmd/x/wasm"
	"github.com/cosmos/gogoproto/proto"
)

type Any struct {
	TypeURL    string `json:"type_url"`
	AppAddress string `json:"app_address"`
	Value      []byte `json:"value"`
}

type AccountSudoMsg struct {
	BeforeTx *BeforeTx `json:"before_tx,omitempty"`
	AfterTx  *AfterTx  `json:"after_tx,omitempty"`
}

type BeforeTx struct {
	Msgs       []*Any `json:"msgs"`
	TxBytes    []byte `json:"tx_bytes"`
	Credential []byte `json:"credential"`
}

type AfterTx struct {
	Success bool `json:"success"`
}

func NewAnyFromProtoMsg(msg proto.Message) (*Any, error) {
	msgBytes, err := proto.Marshal(msg)
	if err != nil {
		return nil, err
	}

	parsed, ok := msg.(*wasm.MsgExecuteContract)
	if ok {
		return &Any{
			TypeURL:    "/" + proto.MessageName(msg),
			AppAddress: parsed.Contract,
			Value:      msgBytes,
		}, nil
	}

	msgAny := &Any{
		TypeURL:    "/" + proto.MessageName(msg),
		AppAddress: "",
		Value:      msgBytes,
	}

	return msgAny, nil
}
