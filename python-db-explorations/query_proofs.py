import sqlite3
from enum import Enum

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
    print("Querying span proofs")
    db_path = "../db/proofs.db"

    start_block = 16843141  # Replace with the desired start block
    for i in range(1000):
    
        proofs = query_span_proofs(db_path, start_block)
    
        for proof in proofs:
            print(f"Proof ID: {proof[0]}, Start Block: {proof[2]}, End Block: {proof[3]}, Status: {proof[4]}, Prover Request ID: {proof[6]}")
        start_block += 1
