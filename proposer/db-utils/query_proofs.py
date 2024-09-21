import sqlite3
from enum import Enum
import os
from dotenv import load_dotenv
import time

class ProofType(Enum):
    SPAN = "SPAN"
    AGG = "AGG"

class ProofStatus(Enum):
    UNREQ = "UNREQ"
    REQ = "REQ"
    COMPLETE = "COMPLETE"
    FAILED = "FAILED"

class ProofRequest:
    id: int
    type: ProofType
    start_block: int
    end_block: int
    status: ProofStatus
    request_added_time: int
    prover_request_id: str
    proof_request_time: int
    l1_block_number: int
    l1_block_hash: str
    proof: bytes

    def __init__(self, id: int, type: ProofType, start_block: int, end_block: int,
                 status: ProofStatus, request_added_time: int, prover_request_id: str,
                 proof_request_time: int, l1_block_number: int, l1_block_hash: str, proof: bytes):
        self.id = id
        self.type = type
        self.start_block = start_block
        self.end_block = end_block
        self.status = status
        self.request_added_time = request_added_time
        self.prover_request_id = prover_request_id
        self.proof_request_time = proof_request_time
        self.l1_block_number = l1_block_number
        self.l1_block_hash = l1_block_hash
        self.proof = proof

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
    proof_requests = []
    if results:
        for result in results:
            proof_request = ProofRequest(
                id=result[0],
                type=ProofType(result[1]),
                start_block=result[2],
                end_block=result[3],
                status=ProofStatus(result[4]),
                request_added_time=result[5],
                prover_request_id=result[6],
                proof_request_time=result[7],
                l1_block_number=result[8],
                l1_block_hash=result[9],
                proof=result[10]
            )
            proof_requests.append(proof_request)
    conn.close()
    
    return proof_requests

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
    # Get all SPAN proofs
    for i in range(20000):
        requests = query_span_proofs(db_path, start_block)
    
        for request in requests:
            print(f"Request ID: {request.id}, Type: {request.type}, Start Block: {request.start_block}, End Block: {request.end_block}, Status: {request.status}, Prover Request ID: {request.prover_request_id}, Time: {request.request_added_time}")
        start_block += 1
    
    # Query for AGG proofs
    agg_proofs = query_agg_proofs(db_path)
    print("\nAGG Proofs:")
    for proof in agg_proofs:
        print(f"Proof ID: {proof[0]}, Type: {proof[1]}, Start Block: {proof[2]}, End Block: {proof[3]}, Status: {proof[4]}, Prover Request ID: {proof[6]}")
    
    # Query for witness generation timeout proofs
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()

    twenty_minutes_ago = int(time.time()) - 20 * 60
    print(f"\nTwenty minutes ago: {twenty_minutes_ago}")
    query = """
    SELECT * FROM proof_requests
    WHERE prover_request_id IS NULL
    AND status = 'REQ'
    AND request_added_time < ?
    """
    cursor.execute(query, (twenty_minutes_ago,))
    
    timeout_proofs = cursor.fetchall()
    conn.close()

    print("\nWitness Generation Timeout Proofs:")
    for proof in timeout_proofs:
        print(f"Proof ID: {proof[0]}, Type: {proof[1]}, Prover Request ID: {proof[6]}, Status: {proof[4]}, Request Added Time: {proof[5]}")