import sqlite3
from enum import Enum
import os
from dotenv import load_dotenv
import time
import sys
from typing import List, Optional
from contextlib import contextmanager

# Types of proofs
class ProofType(Enum):
    SPAN = "SPAN"
    AGG = "AGG"

# Possible statuses for a proof request
class ProofStatus(Enum):
    UNREQ = "UNREQ"
    WITNESSGEN = "WITNESSGEN"
    PROVING = "PROVING"
    COMPLETE = "COMPLETE"
    FAILED = "FAILED"

class DatabaseError(Exception):
    """Custom exception for database errors"""
    pass

@contextmanager
def database_connection(db_path: str):
    """
    A context manager for safe database operations.
    
    Args:
        db_path: Path to the database file
        
    Yields:
        sqlite3.Connection: Database connection object
    
    Raises:
        DatabaseError: In case of connection or database operation issues
    """
    try:
        conn = sqlite3.connect(db_path)
        yield conn
    except sqlite3.Error as e:
        raise DatabaseError(f"Database error occurred: {str(e)}")
    finally:
        conn.close()

# Represents a proof request with all its attributes
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

# Converts a database result to a ProofRequest object
def convert_to_proof_request(result):
    return ProofRequest(
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

# Queries proofs of a specific type from the database
def query_proofs(db_path: str, proof_type: str) -> List[ProofRequest]:
    """
    Requests evidence of a specific type from the database.
    
    Args:
        db_path: Path to the database file
        proof_type: Type of evidence for the query
        
    Returns:
        List[ProofRequest]: List of ProofRequest objects
        
    Raises:
        DatabaseError: In case of issues with the database query
    """
    try:
        with database_connection(db_path) as conn:
            cursor = conn.cursor()
            query = "SELECT * FROM proof_requests WHERE type = ?"
            cursor.execute(query, (proof_type,))
            results = cursor.fetchall()
            return [convert_to_proof_request(result) for result in results] if results else []
    except DatabaseError as e:
        print(f"Error querying proofs: {str(e)}")
        return []

# Queries span proofs from the database
def query_span_proofs(db_path: str) -> List[ProofRequest]:
    """Requests SPAN evidence from the database"""
    return query_proofs(db_path, ProofType.SPAN.value)

# Queries aggregation proofs from the database
def query_agg_proofs(db_path: str) -> List[ProofRequest]:
    """Requests AGG evidence from the database."""
    return query_proofs(db_path, ProofType.AGG.value)


if __name__ == "__main__":
    # Load environment variables from .env file
    load_dotenv()

    # Get L2OO_ADDRESS from environment variables
    L2OO_ADDRESS = os.getenv('L2OO_ADDRESS')
    if L2OO_ADDRESS is None:
        raise ValueError("L2OO_ADDRESS not found in .env file")

    # Get chain ID from command line args
    if len(sys.argv) != 2:
        print("Usage: python query_proofs.py <chain_id>")
        sys.exit(1)
    chain_id = sys.argv[1]

    print(f"L2OO_ADDRESS: {L2OO_ADDRESS}")
    db_path = f"../../db/{chain_id}/proofs.db"

    print(f"DB Path: {db_path}")

    # Get all span proofs
    print("\nSpan Proofs")
    print("-" * 50)
    span_proofs = query_span_proofs(db_path)
    for proof in span_proofs:
        if proof.status is not ProofStatus.UNREQ:
            print(f"Request ID: {proof.id}, Type: {proof.type}, Start Block: {proof.start_block}, End Block: {proof.end_block}, Status: {proof.status}, Prover Request ID: {proof.prover_request_id}, Request Added Time: {proof.request_added_time}, Proof Request Time: {proof.proof_request_time}")
    print("-" * 50)

    # Query for aggregation proofs
    print("\nAggregation Proofs") 
    print("-" * 50)
    agg_proofs = query_agg_proofs(db_path)
    for proof in agg_proofs:
        print(f"Proof ID: {proof.id}, Type: {proof.type}, Start Block: {proof.start_block}, End Block: {proof.end_block}, Status: {proof.status}, Prover Request ID: {proof.prover_request_id}")
    print("-" * 50)
