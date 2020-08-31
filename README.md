# Log-Query
Log-Query provides a way to parse logs, query the parsed data of a log line and template the filtered results of a query of logs. A user can specify a profile to use to parse a log file, provide a query that filters logs based on the intermediate parsed representation, and output the results using custom renderers.

The goal is to provide a set of standard log parsing profiles that can be shared among people with a common log viewing goal in mind (e.g. using a specific profile for debugging a specific issue).


## Parsers
A parser profile is at its core, a regex that describes how to extract information from a single log line. Named capture groups in the regex for a parser will be available as data that can be used in queries using the corresponding key.

### Example
Given the following log line:
```
2020/07/17 23:12:30.037 INFO [Class1] [Thread1] [Application] [] Process snapshot: Snapshotting not enabled
2020/07/17 23:12:30.037 INFO [Class2] [Thread2] [Application] [] Process snapshot: Snapshotting not enabled
```

A parser profile could look like:
```
{
    "parser_name": "Standard Java Log",
    "line_format": "(?P<year>\\d{4})/(?P<month>\\d{2})/(?P<day>\\d{2})\\s(?P<hour>\\d{2}):(?P<minute>\\d{2}):(?P<second>\\d{2})\\.(?P<millisecond>\\d{0,3})\\s(?P<verbosity>\\w+)\\s\\[(?P<class>\\w+)\\]\\s\\[(?P<thread>[^\\]]+)\\]\\s\\[(?P<application>[^\\]]+)\\]\\s\\[(?P<client_id>[^\\]]*)\\]\\s(?P<content>.*)"
}
```
Note the use of double escaping. The regex will essentially construct a mapping from named capture groups to the values for a specific log line.


## Queries
Queries uses a custom syntax to constrain filtering of log lines. Currently filtering decisions are localized to a single log line.

### Syntax
At its core, the syntax reseumbles a conjunctive boolean expression.
```
query :=    atom |
            atom && query
atom := key="value"
```

### Example
Given the sample parser profile and log line defined above, a sample query for the log line could be:
```
verbosity="INFO"&&class="Class1"
```
which would return the first log line.

## OutputGenerator
An output generator takes the intermediate parsed log representation and will output the data in a user defined format.

### JsonGenerator
Using `--json` will output the raw parsed log information to stdout

### HandlebarsGenerator
Using `--handbars format.hbs` will template the format.hbs file with the parsed log information from each log line.

Continuing the running example, consider the result of applying the following template file to the log file with the query specified above:
```
{{verbosity}} -- {{class}}
``` 
produces:
```
INFO -- Class1
```