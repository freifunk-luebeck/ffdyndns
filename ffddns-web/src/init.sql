CREATE TABLE IF NOT EXISTS domains (
	domainname TEXT PRIMARY KEY,
	token TEXT NOT NULL,
	lastupdate TEXT,
	ipv4 TEXT,
	ipv6 TEXT
);
