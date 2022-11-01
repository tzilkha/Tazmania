from typing import List, Optional
from pydantic import BaseModel


class Info(BaseModel):
    fee: str
    relayer_id: str

class Proof(BaseModel):
	None