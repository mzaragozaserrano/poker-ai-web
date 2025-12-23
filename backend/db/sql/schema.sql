-- ============================================================================
-- POKER AI WEB - STAR SCHEMA (DuckDB)
-- ============================================================================
-- Optimizado para:
-- - Ryzen 7 3800X (16 threads)
-- - 64GB RAM (In-Memory Strategy)
-- - Operaciones vectorizadas SIMD
-- - Consultas analíticas de winrate/ROI
-- ============================================================================

-- ============================================================================
-- 1. IDENTIDAD Y JUGADORES
-- ============================================================================

-- Tabla: players
-- Consolidación de identidad única para el usuario y oponentes
CREATE TABLE IF NOT EXISTS players (
    player_id VARCHAR PRIMARY KEY,
    display_name VARCHAR NOT NULL,
    is_hero BOOLEAN NOT NULL DEFAULT FALSE,
    notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Índice para filtros rápidos de Hero
CREATE INDEX IF NOT EXISTS idx_players_is_hero 
ON players(is_hero) 
WHERE is_hero = TRUE;

-- Tabla: player_aliases
-- Gestión de múltiples nicknames y cuentas (multi-sala)
CREATE TABLE IF NOT EXISTS player_aliases (
    alias_id VARCHAR PRIMARY KEY,
    player_id VARCHAR NOT NULL,
    site_name VARCHAR NOT NULL,
    site_nickname VARCHAR NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Índice compuesto para búsqueda rápida por sala y nickname
CREATE UNIQUE INDEX IF NOT EXISTS idx_player_aliases_site_nickname 
ON player_aliases(site_name, site_nickname);

-- Índice para joins con players
CREATE INDEX IF NOT EXISTS idx_player_aliases_player_id 
ON player_aliases(player_id);

-- ============================================================================
-- 2. ESTRUCTURA ANALÍTICA (STAR SCHEMA)
-- ============================================================================

-- Tabla: hands_metadata (DIMENSION TABLE)
-- Metadata de cada mano para análisis dimensional
CREATE TABLE IF NOT EXISTS hands_metadata (
    hand_id VARCHAR PRIMARY KEY,
    session_id VARCHAR,
    tournament_id VARCHAR,
    timestamp TIMESTAMP NOT NULL,
    stake VARCHAR NOT NULL,
    format VARCHAR NOT NULL CHECK (format IN ('CASH', 'MTT', 'SNG', 'EXPRESSO')),
    table_name VARCHAR NOT NULL,
    blind_level BIGINT NOT NULL,
    button_seat UTINYINT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Índice B-Tree para filtros temporales (crítico para análisis de sesiones)
CREATE INDEX IF NOT EXISTS idx_hands_metadata_timestamp 
ON hands_metadata(timestamp);

-- Índice para filtros por formato
CREATE INDEX IF NOT EXISTS idx_hands_metadata_format 
ON hands_metadata(format);

-- Índice para filtros por stake
CREATE INDEX IF NOT EXISTS idx_hands_metadata_stake 
ON hands_metadata(stake);

-- Índice para joins con sesiones
CREATE INDEX IF NOT EXISTS idx_hands_metadata_session_id 
ON hands_metadata(session_id);

-- Tabla: hands_actions (FACT TABLE)
-- Tabla de hechos con todas las acciones de todas las manos
CREATE TABLE IF NOT EXISTS hands_actions (
    action_id VARCHAR PRIMARY KEY,
    hand_id VARCHAR NOT NULL,
    player_id VARCHAR NOT NULL,
    street VARCHAR NOT NULL CHECK (street IN ('PREFLOP', 'FLOP', 'TURN', 'RIVER')),
    action_type VARCHAR NOT NULL CHECK (action_type IN ('FOLD', 'CALL', 'RAISE', 'BET', 'CHECK', 'ALL_IN')),
    amount_cents BIGINT NOT NULL DEFAULT 0,
    is_hero_action BOOLEAN NOT NULL DEFAULT FALSE,
    ev_cents BIGINT,
    action_sequence INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Índice para joins ultra-rápidos con hands_metadata
CREATE INDEX IF NOT EXISTS idx_hands_actions_hand_id 
ON hands_actions(hand_id);

-- Índice compuesto para cálculo de estadísticas (VPIP, PFR, etc.)
CREATE INDEX IF NOT EXISTS idx_hands_actions_player_street 
ON hands_actions(player_id, street);

-- Índice para filtros de Hero
CREATE INDEX IF NOT EXISTS idx_hands_actions_is_hero 
ON hands_actions(is_hero_action) 
WHERE is_hero_action = TRUE;

-- Índice para análisis por tipo de acción
CREATE INDEX IF NOT EXISTS idx_hands_actions_action_type 
ON hands_actions(action_type);

-- ============================================================================
-- 3. ECONOMÍA Y RESULTADOS
-- ============================================================================

-- Tabla: cash_sessions
-- Sesiones de cash game con métricas de rendimiento
CREATE TABLE IF NOT EXISTS cash_sessions (
    session_id VARCHAR PRIMARY KEY,
    player_id VARCHAR NOT NULL,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP,
    stake VARCHAR NOT NULL,
    net_won_cents BIGINT NOT NULL DEFAULT 0,
    ev_won_cents BIGINT NOT NULL DEFAULT 0,
    rake_cents BIGINT NOT NULL DEFAULT 0,
    rakeback_cents BIGINT NOT NULL DEFAULT 0,
    bb_100 DOUBLE,
    ev_bb_100 DOUBLE,
    hands_played INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Índice para filtros temporales de sesiones
CREATE INDEX IF NOT EXISTS idx_cash_sessions_start_time 
ON cash_sessions(start_time);

-- Índice para joins con players
CREATE INDEX IF NOT EXISTS idx_cash_sessions_player_id 
ON cash_sessions(player_id);

-- Índice para filtros por stake
CREATE INDEX IF NOT EXISTS idx_cash_sessions_stake 
ON cash_sessions(stake);

-- Tabla: tournaments
-- Información de torneos
CREATE TABLE IF NOT EXISTS tournaments (
    tournament_id VARCHAR PRIMARY KEY,
    tournament_name VARCHAR NOT NULL,
    format VARCHAR NOT NULL CHECK (format IN ('MTT', 'SNG', 'EXPRESSO')),
    buyin_cents BIGINT NOT NULL,
    rake_cents BIGINT NOT NULL,
    total_entries INTEGER,
    prize_pool_cents BIGINT,
    start_time TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Índice para filtros temporales
CREATE INDEX IF NOT EXISTS idx_tournaments_start_time 
ON tournaments(start_time);

-- Índice para filtros por formato
CREATE INDEX IF NOT EXISTS idx_tournaments_format 
ON tournaments(format);

-- Tabla: tournament_results
-- Resultados de torneos para cálculo de ROI
CREATE TABLE IF NOT EXISTS tournament_results (
    result_id VARCHAR PRIMARY KEY,
    tournament_id VARCHAR NOT NULL,
    player_id VARCHAR NOT NULL,
    finish_position INTEGER NOT NULL,
    prize_won_cents BIGINT NOT NULL DEFAULT 0,
    bounty_won_cents BIGINT NOT NULL DEFAULT 0,
    total_won_cents BIGINT NOT NULL DEFAULT 0,
    roi_real DOUBLE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Índice para joins con tournaments
CREATE INDEX IF NOT EXISTS idx_tournament_results_tournament_id 
ON tournament_results(tournament_id);

-- Índice para joins con players
CREATE INDEX IF NOT EXISTS idx_tournament_results_player_id 
ON tournament_results(player_id);

-- ============================================================================
-- 4. CONFIGURACIÓN DE DUCKDB PARA OPTIMIZACIÓN
-- ============================================================================

-- Configurar DuckDB para aprovechar los 16 threads del Ryzen 3800X
PRAGMA threads=16;

-- Configurar memoria máxima (usar 48GB de los 64GB disponibles)
PRAGMA memory_limit='48GB';

-- Habilitar optimizaciones de vectorización
PRAGMA enable_object_cache=true;

-- ============================================================================
-- FIN DEL ESQUEMA
-- ============================================================================

