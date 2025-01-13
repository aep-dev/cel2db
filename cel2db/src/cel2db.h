// C bindings for cel2db. These are used to export the function to
// other languages.
#ifndef CEL2DB_H
#define CEL2DB_H

char *cel_to_sql(const char *cel_expr, const char *sql_dialect);

#endif