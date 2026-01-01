"""
Modelos Pydantic para cálculos de equity.
"""

from typing import List, Optional
from pydantic import BaseModel, Field, field_validator


class EquityCalculateRequest(BaseModel):
    """Request para calcular equity entre dos manos."""
    
    hero_cards: str = Field(
        ..., 
        min_length=4,
        max_length=4,
        description="Cartas del héroe (ej: 'AhKd')",
        examples=["AhKd", "QsQh"]
    )
    villain_cards: str = Field(
        ..., 
        min_length=4,
        max_length=4,
        description="Cartas del villano (ej: 'QsQh')",
        examples=["QsQh", "8c8d"]
    )
    board: str = Field(
        "", 
        description="Cartas comunitarias (0, 3, 4 o 5 cartas)",
        examples=["", "Qh7s2c", "Qh7s2cKd", "Qh7s2cKd3h"]
    )
    iterations: int = Field(
        100000, 
        ge=1000, 
        le=1000000,
        description="Número de simulaciones Monte Carlo"
    )
    
    @field_validator('hero_cards', 'villain_cards')
    @classmethod
    def validate_cards_format(cls, v):
        """Valida que las cartas tengan formato correcto."""
        if len(v) != 4:
            raise ValueError("Las cartas deben ser exactamente 2 (formato: XxYy)")
        
        # Validar formato básico
        ranks = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']
        suits = ['h', 'd', 'c', 's']
        
        for i in range(0, len(v), 2):
            if i + 1 >= len(v):
                break
            rank = v[i]
            suit = v[i + 1]
            
            if rank not in ranks or suit not in suits:
                raise ValueError(
                    f"Carta inválida: '{rank}{suit}'. "
                    f"Formato: Rank(A-2)Suit(h/d/c/s)"
                )
        
        return v
    
    @field_validator('board')
    @classmethod
    def validate_board(cls, v):
        """Valida que el board tenga 0, 3, 4 o 5 cartas."""
        if not v:
            return v
        
        if len(v) not in [0, 6, 8, 10]:  # 0, 3*2, 4*2, 5*2
            raise ValueError(
                f"Board debe tener 0, 3, 4 o 5 cartas. Recibido: {len(v)//2} cartas"
            )
        
        return v
    
    class Config:
        json_schema_extra = {
            "example": {
                "hero_cards": "AhKd",
                "villain_cards": "QsQh",
                "board": "Qh7s2c",
                "iterations": 50000
            }
        }


class EquityCalculateResponse(BaseModel):
    """Response con resultados del cálculo de equity."""
    
    hero_equity: float = Field(..., ge=0, le=1, description="Equity del héroe (0.0 - 1.0)")
    villain_equity: float = Field(..., ge=0, le=1, description="Equity del villano (0.0 - 1.0)")
    tie_equity: float = Field(..., ge=0, le=1, description="Probabilidad de empate (0.0 - 1.0)")
    hero_percent: float = Field(..., ge=0, le=100, description="Equity del héroe en porcentaje")
    villain_percent: float = Field(..., ge=0, le=100, description="Equity del villano en porcentaje")
    simulations_run: int = Field(..., ge=0, description="Simulaciones ejecutadas")
    converged_early: bool = Field(..., description="Si convergió antes de completar")
    standard_error: float = Field(..., ge=0, description="Error estándar estimado")
    elapsed_ms: Optional[int] = Field(None, ge=0, description="Tiempo de cálculo en ms")
    
    class Config:
        json_schema_extra = {
            "example": {
                "hero_equity": 0.82,
                "villain_equity": 0.18,
                "tie_equity": 0.00,
                "hero_percent": 82.0,
                "villain_percent": 18.0,
                "simulations_run": 50000,
                "converged_early": False,
                "standard_error": 0.002,
                "elapsed_ms": 85
            }
        }


class EquityMultiwayRequest(BaseModel):
    """Request para cálculo multiway (3+ jugadores)."""
    
    hands: List[str] = Field(
        ..., 
        min_length=2,
        max_length=10,
        description="Lista de manos para cada jugador"
    )
    board: str = Field("", description="Cartas comunitarias")
    iterations: int = Field(
        50000, 
        ge=1000, 
        le=500000,
        description="Número de simulaciones"
    )
    
    @field_validator('hands')
    @classmethod
    def validate_hands(cls, v):
        """Valida que todas las manos tengan formato correcto."""
        if len(v) < 2:
            raise ValueError("Se necesitan al menos 2 manos")
        
        for hand in v:
            if len(hand) != 4:
                raise ValueError(f"Mano inválida: '{hand}'. Debe tener 4 caracteres")
        
        return v
    
    class Config:
        json_schema_extra = {
            "example": {
                "hands": ["AhKd", "QsQh", "8c8d"],
                "board": "",
                "iterations": 50000
            }
        }

