-- Your SQL goes here
CREATE TABLE currency (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL
);

CREATE TABLE wallet (
  user_id INTEGER NOT NULL,
  currency_id INTEGER NOT NULL,
  quantity INTEGER NOT NULL DEFAULT 1,
  PRIMARY KEY (user_id, currency_id),
  FOREIGN KEY (user_id) REFERENCES users(id),
  FOREIGN KEY (currency_id) REFERENCES currency(id)
);