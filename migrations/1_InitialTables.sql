        CREATE TABLE IF NOT EXISTS users(
                email VARCHAR(100) unique,
                username VARCHAR(50) unique,
                hashed_password VARCHAR(100),
                login_attempts INTEGER DEFAULT 0,
                auth_level INTEGER DEFAULT 0,
                PRIMARY KEY(email,username)
        );

        CREATE TABLE IF NOT EXISTS sessions(
            session_key VARCHAR(100),
            username VARHCAR(50),
            expiry INTEGER,
            PRIMARY KEY(session_key)
        );

       CREATE TABLE IF NOT EXISTS codes(
            id INTEGER AUTO_INCREMENT PRIMARY KEY,
            code_type VARCHAR(20),
            email VARCHAR(100),
            code VARCHAR(30),
            created_ts VARCHAR(30),
            expiry_ts VARCHAR(30),
            used INTEGER default 0
        );
