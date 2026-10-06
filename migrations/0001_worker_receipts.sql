-- SPDX-License-Identifier: Apache-2.0
CREATE TABLE IF NOT EXISTS worker_schema (version INTEGER PRIMARY KEY CHECK(version=1));
INSERT OR IGNORE INTO worker_schema VALUES(1);
CREATE TABLE IF NOT EXISTS conversations (
    invalidated INTEGER NOT NULL DEFAULT 0 CHECK(invalidated IN (0,1)),
 room_id TEXT NOT NULL, agent_id TEXT NOT NULL, conversation_id TEXT NOT NULL,
 generation INTEGER NOT NULL CHECK(generation>0), policy_revision TEXT NOT NULL,
 guidance_revision TEXT NOT NULL, thread_id TEXT, consumed_revision INTEGER NOT NULL DEFAULT 0,
 state TEXT NOT NULL CHECK(state IN ('reserved','running','pending_ack','ready','unusable')),
 PRIMARY KEY(room_id,agent_id), UNIQUE(conversation_id)
);
CREATE TABLE IF NOT EXISTS receipts (
 task_id TEXT PRIMARY KEY, room_id TEXT NOT NULL, agent_id TEXT NOT NULL,
 conversation_id TEXT NOT NULL, generation INTEGER NOT NULL, fingerprint TEXT NOT NULL,
 context_digest TEXT NOT NULL, base_revision INTEGER NOT NULL, end_revision INTEGER NOT NULL,
 policy_revision TEXT NOT NULL, guidance_revision TEXT NOT NULL, source_ids TEXT NOT NULL,
 phase TEXT NOT NULL CHECK(phase IN ('reserved','running','completed','failed','interrupted')),
 prior_ready INTEGER NOT NULL, thread_id TEXT, turn_id TEXT, result TEXT, error_code TEXT, ack TEXT
);
CREATE INDEX IF NOT EXISTS receipt_generation ON receipts(conversation_id,generation);
