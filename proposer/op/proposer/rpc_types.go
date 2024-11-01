package proposer

type SpanProofRequest struct {
	Start uint64 `json:"start"`
	End   uint64 `json:"end"`
}

type AggProofRequest struct {
	Subproofs [][]byte `json:"subproofs"`
	L1Head    string   `json:"head"`
}

type ValidateConfigRequest struct {
	Address string `json:"address"`
}

type ValidateConfigResponse struct {
	RollupConfigHashValid bool `json:"rollup_config_hash_valid"`
	AggVkeyValid          bool `json:"agg_vkey_valid"`
	RangeVkeyValid        bool `json:"range_vkey_valid"`
}

// WitnessGenerationResponse is the response type for the `request_span_proof` and `request_agg_proof`
// RPCs from the op-succinct-server.
type WitnessGenerationResponse struct {
	ProofID string `json:"proof_id"`
}

// UnclaimDescription is the description of why a proof was unclaimed.
type UnclaimDescription int

const (
	UnexpectedProverError UnclaimDescription = iota
	ProgramExecutionError
	CycleLimitExceeded
	Other
)

// SP1ProofStatus represents the status of a proof in the SP1 network.
type SP1ProofStatus int

const (
	SP1ProofStatusUnknown SP1ProofStatus = iota
	SP1ProofStatusPending
	SP1ProofStatusInProgress
	SP1ProofStatusFulfilled
	SP1ProofStatusUnclaimed
	SP1ProofStatusInvalid
)

// ProofStatusResponse is the response type for the `GetProofStatus` RPC from the op-succinct-server.
type ProofStatusResponse struct {
	Status             SP1ProofStatus     `json:"status"`
	Proof              []byte             `json:"proof"`
	UnclaimDescription UnclaimDescription `json:"unclaim_description,omitempty"`
}
