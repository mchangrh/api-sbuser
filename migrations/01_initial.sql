CREATE TABLE userid (
    sbid VARCHAR(64) UNIQUE NOT NULL,
    discord INTEGER UNIQUE NOT NULL,
    locked BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (discord)
);
CREATE UNIQUE INDEX idx_userid_sbid ON userid (sbid);