-- Your SQL goes here
CREATE TABLE Settings(
    chat_id BIGINT NOT NULL,
    module_identifier TEXT NOT NULL,
    enabled BOOLEAN NOT null DEFAULT false,
    settings JSON NOT NULL DEFAULT '{}',
    PRIMARY KEY (chat_id, module_identifier)
);

CREATE TABLE ModuleData(
    module_identifier TEXT NOT NULL,
    data_key TEXT NOT NULL,
    data JSON NOT NULL DEFAULT '{}',
    PRIMARY KEY (module_identifier, data_key)
);