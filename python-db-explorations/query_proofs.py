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
        # print(f"ID: {first_result[0]}")
        # print(f"Type: {first_result[1]}")
        # print(f"Start Block: {first_result[2]}")
        # print(f"End Block: {first_result[3]}")
        # print(f"Status: {first_result[4]}")
        # print(f"Request Added Time: {first_result[5]}")
        # print(f"Prover Request ID: {first_result[6]}")
        # print(f"Proof Request Time: {first_result[7]}")
        # print(f"L1 Block Number: {first_result[8]}")
        # print(f"L1 Block Hash: {first_result[9]}")
        # print(f"Proof: {first_result[10]}")

    # print("Results:", results)
    conn.close()
    
    return results

if __name__ == "__main__":
    # Load environment variables from .env file
    load_dotenv()

    # Get L2OO_ADDRESS from environment variables
    L2OO_ADDRESS = os.getenv('L2OO_ADDRESS')
    if L2OO_ADDRESS is None:
        raise ValueError("L2OO_ADDRESS not found in .env file")

    print(f"L2OO_ADDRESS: {L2OO_ADDRESS}")
    print("Querying span proofs")
    db_path = "../db/proofs.db"

    start_block = 17048966  # Replace with the desired start block
    for i in range(3000):
    
        proofs = query_span_proofs(db_path, start_block)
    
        for proof in proofs:
            print(f"Proof ID: {proof[0]}, Type: {proof[1]}, Start Block: {proof[2]}, End Block: {proof[3]}, Status: {proof[4]}, Prover Request ID: {proof[6]}")
        start_block += 1
