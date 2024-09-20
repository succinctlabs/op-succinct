import sqlite3
from enum import Enum
import os
from dotenv import load_dotenv

class ProofType(Enum):
    SPAN = 1
    AGG = 2

class ProofStatus(Enum):
    UNREQ = 1
    REQ = 2
    COMPLETE = 3
    FAILED = 4

def query_span_proofs(db_path, start_block):
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()

    # query = """
    # SELECT * FROM proof_requests
    # WHERE type = ? AND status = ? AND start_block = ?
    # """
    # cursor.execute(query, (ProofType.SPAN.value, ProofStatus.COMPLETE.value, start_block))

    query = """
    SELECT * FROM proof_requests
    WHERE type = 'SPAN' AND start_block = ?
    """
    cursor.execute(query, (start_block,))
    
    results = cursor.fetchall()
    if results:
        first_result = results[0]
        # ProofRequest struct from proofrequest.go:
        # type ProofRequest struct {
        #     ID               int                `json:"id,omitempty"`
        #     Type             proofrequest.Type  `json:"type,omitempty"`
        #     StartBlock       uint64             `json:"start_block,omitempty"`
        #     EndBlock         uint64             `json:"end_block,omitempty"`
        #     Status           proofrequest.Status `json:"status,omitempty"`
        #     RequestAddedTime uint64             `json:"request_added_time,omitempty"`
        #     ProverRequestID  string             `json:"prover_request_id,omitempty"`
        #     ProofRequestTime uint64             `json:"proof_request_time,omitempty"`
        #     L1BlockNumber    uint64             `json:"l1_block_number,omitempty"`
        #     L1BlockHash      string             `json:"l1_block_hash,omitempty"`
        #     Proof            []byte             `json:"proof,omitempty"`
        # }

    conn.close()
    
    return results

def query_agg_proofs(db_path):
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()

    query = """
    SELECT * FROM proof_requests
    WHERE type = 'AGG'
    """
    cursor.execute(query)
    
    results = cursor.fetchall()
    conn.close()
    
    return results

def get_earliest_span_start_block(db_path):
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()

    query = """
    SELECT MIN(start_block) FROM proof_requests
    WHERE type = 'SPAN'
    """
    cursor.execute(query)
    
    result = cursor.fetchone()[0]
    conn.close()
    
    return result


if __name__ == "__main__":
    # Load environment variables from .env file
    load_dotenv()

    # Get L2OO_ADDRESS from environment variables
    L2OO_ADDRESS = os.getenv('L2OO_ADDRESS')
    if L2OO_ADDRESS is None:
        raise ValueError("L2OO_ADDRESS not found in .env file")

    print(f"L2OO_ADDRESS: {L2OO_ADDRESS}")
    print("\nQuerying span proofs")
    db_path = "../../db/proofs.db"

    earliest_start_block = get_earliest_span_start_block(db_path)
    print(f"\nEarliest span proof start block: {earliest_start_block}")

    start_block = earliest_start_block
    for i in range(4000):
        proofs = query_span_proofs(db_path, start_block)
    
        for proof in proofs:
            print(f"Proof ID: {proof[0]}, Type: {proof[1]}, Start Block: {proof[2]}, End Block: {proof[3]}, Status: {proof[4]}, Prover Request ID: {proof[6]}")
        start_block += 1
    
    # Query for AGG proofs
    agg_proofs = query_agg_proofs(db_path)
    print("\nAGG Proofs:")
    for proof in agg_proofs:
        print(f"Proof ID: {proof[0]}, Type: {proof[1]}, Start Block: {proof[2]}, End Block: {proof[3]}, Status: {proof[4]}, Prover Request ID: {proof[6]}")