import uvicorn
from fastapi import Depends, FastAPI, HTTPException
from fastapi.encoders import jsonable_encoder
from fastapi.responses import JSONResponse

import schemas
from config import CONFIG
from network import ContractCalls

from tinydb import TinyDB, Query
from pathlib import Path
import os

FIRST_BLOCK = "0"

near = ContractCalls(
	CONFIG['tazmania_id'], 
	CONFIG['relayer_id'], 
	CONFIG['relayer_key'], 
	"https://rpc.testnet.near.org"
)

app = FastAPI(title="Tazmania Relayer",
              description="Tazmania relaying server for accepting and submitting withdraw requests.",
              version="1.0.0", )

# For API Exceptions

@app.exception_handler(Exception)
def validation_exception_handler(request, err):
    base_error_message = f"Failed to execute: {request.method}: {request.url}"
    return JSONResponse(status_code=400, content={"message": f"{base_error_message}. Detail: {err}"})

# Get information
@app.get('/info', tags=["Info"], response_model=schemas.Info, status_code=201)
async def relayer_info():
    """
    Return relayer information
    """

    out = near.n_leaves()
    print(out)

    out = near.get_leaves()
    print(out)

    return {"fee": CONFIG['fee'], "relayer_id": CONFIG['relayer_id']}

@app.get('/proof', tags=['Proof'], response_model=schemas.Proof, status_code=201)
async def merkle_parameters():
	return {}



if __name__ == "__main__":



	print("Local Storage")
	my_file = Path("db.json")
	if not my_file.is_file():
		fp = open(my_file, 'x')
		fp.close()
		print("No local db found.. Creating new.")
	db = TinyDB('db.json')
	print("DB loaded.")

	uvicorn.run("server:app", port=9000, reload=True)
